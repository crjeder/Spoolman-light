use chrono::Utc;
use rand::Rng;
use spoolman_types::{
    models::{DataStore, Filament, Location, Spool, StoreMeta},
    requests::{CreateFilament, CreateLocation, CreateSpool, UpdateFilament, UpdateLocation, UpdateSpool},
    responses::{LocationResponse, SpoolResponse},
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("not found")]
    NotFound,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, StoreError>;

/// Thread-safe JSON-backed data store.
#[derive(Clone)]
pub struct JsonStore {
    inner: Arc<RwLock<DataStore>>,
    path: PathBuf,
}

impl JsonStore {
    /// Load from disk, or create an empty store if the file doesn't exist.
    ///
    /// `path` MUST come from operator-controlled configuration (e.g. the
    /// `SPOOLMAN_DATA_FILE` environment variable), never from HTTP request data.
    pub fn load(path: &Path) -> Result<Self> {
        // Resolve to a canonical absolute path to prevent `..` traversal and
        // symlink-based escapes.  When the file does not yet exist we
        // canonicalize only the parent directory (creating it first if needed)
        // and append the filename so the resolved path is still fully anchored.
        let resolved = Self::resolve_data_path(path)?;
        let data = if resolved.exists() {
            // `resolved` is the canonicalized output of resolve_data_path; the
            // original path is operator-configured (env var), not user input.
            let contents = std::fs::read_to_string(&resolved)?; // nosemgrep: path-traversal
            serde_json::from_str(&contents)?
        } else {
            DataStore {
                meta: StoreMeta { schema_version: 1 },
                ..Default::default()
            }
        };
        Ok(Self {
            inner: Arc::new(RwLock::new(data)),
            path: resolved,
        })
    }

    /// Resolve `path` to a canonicalized, absolute `PathBuf`.
    ///
    /// Returns an error if the path contains `..` components or if the parent
    /// directory cannot be created / resolved.
    fn resolve_data_path(path: &Path) -> Result<PathBuf> {
        // Reject literal `..` components before any resolution.
        for component in path.components() {
            use std::path::Component;
            if matches!(component, Component::ParentDir) {
                return Err(StoreError::Validation(
                    "data file path must not contain '..' components".into(),
                ));
            }
        }
        // Obtain a fully resolved path so symlinks cannot be used to escape.
        let resolved = if path.exists() {
            path.canonicalize()?
        } else {
            // File doesn't exist yet; resolve the parent directory first.
            let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
            let canon_parent = match parent {
                Some(p) => {
                    std::fs::create_dir_all(p)?;
                    p.canonicalize()?
                }
                None => std::env::current_dir()?,
            };
            let filename = path
                .file_name()
                .ok_or_else(|| StoreError::Validation("data file path has no filename".into()))?;
            canon_parent.join(filename)
        };
        Ok(resolved)
    }

    /// Atomically write the store to disk (write to .tmp, then rename).
    fn flush(&self, store: &DataStore) -> Result<()> {
        let tmp = self.path.with_extension("json.tmp");
        let json = serde_json::to_string_pretty(store)?;
        // Ensure parent directory exists
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&tmp, &json)?;
        std::fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    /// Generate a random u32 ID not already in `existing`.
    fn new_id(existing: &HashSet<u32>) -> u32 {
        let mut rng = rand::thread_rng();
        loop {
            let id: u32 = rng.gen_range(1..=u32::MAX);
            if !existing.contains(&id) {
                return id;
            }
        }
    }

    // ── Filament ───────────────────────────────────────────────────────────────

    pub fn list_filaments(
        &self,
        material: Option<&str>,
        sort: Option<&str>,
        order: Option<&str>,
        offset: usize,
        limit: Option<usize>,
    ) -> Result<(Vec<Filament>, usize)> {
        let store = self.inner.read().unwrap();
        let mut items: Vec<Filament> = store
            .filaments
            .iter()
            .filter(|f| {
                if let Some(m) = material {
                    f.material.as_ref().map(|mt| mt.abbreviation()) == Some(m)
                } else {
                    true
                }
            })
            .cloned()
            .collect();

        sort_items(&mut items, sort, order, |f, field| match field {
            "manufacturer" => f.manufacturer.as_deref().unwrap_or("").to_string(),
            "material" => f.material.as_ref().map(|m| m.abbreviation()).unwrap_or("").to_string(),
            "registered" => f.registered.to_rfc3339(),
            _ => f.registered.to_rfc3339(),
        });

        let total = items.len();
        let items = paginate(items, offset, limit);
        Ok((items, total))
    }

    pub fn get_filament(&self, id: u32) -> Result<Filament> {
        let store = self.inner.read().unwrap();
        store
            .filaments
            .iter()
            .find(|f| f.id == id)
            .cloned()
            .ok_or(StoreError::NotFound)
    }

    pub fn create_filament(&self, req: CreateFilament) -> Result<Filament> {
        let mut store = self.inner.write().unwrap();
        let existing: HashSet<u32> = store.filaments.iter().map(|f| f.id).collect();
        let id = Self::new_id(&existing);
        let filament = Filament {
            id,
            manufacturer: req.manufacturer,
            material: req.material,
            material_modifier: req.material_modifier,
            diameter: req.diameter,
            net_weight: req.net_weight,
            density: req.density,
            print_temp: req.print_temp,
            bed_temp: req.bed_temp,
            spool_weight: req.spool_weight,
            min_print_temp: req.min_print_temp,
            max_print_temp: req.max_print_temp,
            min_bed_temp: req.min_bed_temp,
            max_bed_temp: req.max_bed_temp,
            registered: Utc::now(),
            comment: req.comment,
        };
        store.filaments.push(filament.clone());
        self.flush(&store)?;
        Ok(filament)
    }

    pub fn update_filament(&self, id: u32, req: UpdateFilament) -> Result<Filament> {
        let mut store = self.inner.write().unwrap();
        let filament = store
            .filaments
            .iter_mut()
            .find(|f| f.id == id)
            .ok_or(StoreError::NotFound)?;
        apply_option(&mut filament.manufacturer, req.manufacturer);
        apply_option(&mut filament.material, req.material);
        apply_option(&mut filament.material_modifier, req.material_modifier);
        if let Some(v) = req.diameter { filament.diameter = v; }
        apply_option(&mut filament.net_weight, req.net_weight);
        if let Some(v) = req.density { filament.density = v; }
        apply_option(&mut filament.print_temp, req.print_temp);
        apply_option(&mut filament.bed_temp, req.bed_temp);
        apply_option(&mut filament.spool_weight, req.spool_weight);
        apply_option(&mut filament.min_print_temp, req.min_print_temp);
        apply_option(&mut filament.max_print_temp, req.max_print_temp);
        apply_option(&mut filament.min_bed_temp, req.min_bed_temp);
        apply_option(&mut filament.max_bed_temp, req.max_bed_temp);
        apply_option(&mut filament.comment, req.comment);
        let filament = filament.clone();
        self.flush(&store)?;
        Ok(filament)
    }

    pub fn delete_filament(&self, id: u32) -> Result<()> {
        let mut store = self.inner.write().unwrap();
        // Referential integrity check
        let referencing: Vec<u32> = store
            .spools
            .iter()
            .filter(|s| s.filament_id == id)
            .map(|s| s.id)
            .collect();
        if !referencing.is_empty() {
            return Err(StoreError::Conflict(format!(
                "filament is referenced by spools: {:?}",
                referencing
            )));
        }
        let before = store.filaments.len();
        store.filaments.retain(|f| f.id != id);
        if store.filaments.len() == before {
            return Err(StoreError::NotFound);
        }
        self.flush(&store)?;
        Ok(())
    }

    // ── Spool ──────────────────────────────────────────────────────────────────

    pub fn list_spools(
        &self,
        filament_id: Option<u32>,
        location_id: Option<u32>,
        allow_archived: bool,
        sort: Option<&str>,
        order: Option<&str>,
        offset: usize,
        limit: Option<usize>,
    ) -> Result<(Vec<SpoolResponse>, usize)> {
        let store = self.inner.read().unwrap();
        let filament_map: HashMap<u32, Filament> =
            store.filaments.iter().map(|f| (f.id, f.clone())).collect();

        let mut items: Vec<SpoolResponse> = store
            .spools
            .iter()
            .filter(|s| {
                if !allow_archived && s.archived {
                    return false;
                }
                if let Some(fid) = filament_id {
                    if s.filament_id != fid {
                        return false;
                    }
                }
                if let Some(lid) = location_id {
                    if s.location_id != Some(lid) {
                        return false;
                    }
                }
                true
            })
            .filter_map(|s| {
                filament_map
                    .get(&s.filament_id)
                    .map(|f| SpoolResponse::new(s.clone(), f.clone()))
            })
            .collect();

        sort_items(&mut items, sort, order, |s, field| match field {
            "registered" => s.spool.registered.to_rfc3339(),
            "last_used" => s
                .spool
                .last_used
                .map(|d| d.to_rfc3339())
                .unwrap_or_default(),
            _ => s.spool.registered.to_rfc3339(),
        });

        let total = items.len();
        let items = paginate(items, offset, limit);
        Ok((items, total))
    }

    pub fn get_spool(&self, id: u32) -> Result<SpoolResponse> {
        let store = self.inner.read().unwrap();
        let spool = store
            .spools
            .iter()
            .find(|s| s.id == id)
            .ok_or(StoreError::NotFound)?;
        let filament = store
            .filaments
            .iter()
            .find(|f| f.id == spool.filament_id)
            .ok_or(StoreError::NotFound)?;
        Ok(SpoolResponse::new(spool.clone(), filament.clone()))
    }

    pub fn create_spool(&self, req: CreateSpool) -> Result<SpoolResponse> {
        let mut store = self.inner.write().unwrap();
        // Validate filament exists
        let filament = store
            .filaments
            .iter()
            .find(|f| f.id == req.filament_id)
            .ok_or(StoreError::NotFound)?
            .clone();
        let existing: HashSet<u32> = store.spools.iter().map(|s| s.id).collect();
        let id = Self::new_id(&existing);
        let spool = Spool {
            id,
            filament_id: req.filament_id,
            location_id: req.location_id,
            colors: req.colors,
            color_name: req.color_name,
            initial_weight: req.initial_weight,
            current_weight: req.initial_weight,
            registered: Utc::now(),
            first_used: req.first_used,
            last_used: req.last_used,
            comment: req.comment,
            archived: false,
        };
        store.spools.push(spool.clone());
        self.flush(&store)?;
        Ok(SpoolResponse::new(spool, filament))
    }

    pub fn update_spool(&self, id: u32, req: UpdateSpool) -> Result<SpoolResponse> {
        let mut store = self.inner.write().unwrap();
        let spool = store
            .spools
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or(StoreError::NotFound)?;

        let weight_changed = req.current_weight.is_some();
        if let Some(colors) = req.colors { spool.colors = colors; }
        apply_option_nullable(&mut spool.color_name, req.color_name);
        apply_option_nullable_u32(&mut spool.location_id, req.location_id);
        if let Some(w) = req.current_weight { spool.current_weight = w; }
        apply_option_nullable_dt(&mut spool.first_used, req.first_used);
        apply_option_nullable_dt(&mut spool.last_used, req.last_used);
        apply_option_nullable(&mut spool.comment, req.comment);
        if let Some(archived) = req.archived { spool.archived = archived; }
        if weight_changed && req.last_used.is_none() {
            spool.last_used = Some(Utc::now());
        }
        let spool = spool.clone();
        let filament = store
            .filaments
            .iter()
            .find(|f| f.id == spool.filament_id)
            .ok_or(StoreError::NotFound)?
            .clone();
        self.flush(&store)?;
        Ok(SpoolResponse::new(spool, filament))
    }

    pub fn delete_spool(&self, id: u32) -> Result<()> {
        let mut store = self.inner.write().unwrap();
        let before = store.spools.len();
        store.spools.retain(|s| s.id != id);
        if store.spools.len() == before {
            return Err(StoreError::NotFound);
        }
        self.flush(&store)?;
        Ok(())
    }

    pub fn clone_spool(&self, id: u32) -> Result<SpoolResponse> {
        let (spool, filament) = {
            let store = self.inner.read().unwrap();
            let spool = store
                .spools
                .iter()
                .find(|s| s.id == id)
                .ok_or(StoreError::NotFound)?
                .clone();
            let filament = store
                .filaments
                .iter()
                .find(|f| f.id == spool.filament_id)
                .ok_or(StoreError::NotFound)?
                .clone();
            (spool, filament)
        };

        let mut store = self.inner.write().unwrap();
        let existing: HashSet<u32> = store.spools.iter().map(|s| s.id).collect();
        let new_id = Self::new_id(&existing);
        let cloned = Spool {
            id: new_id,
            filament_id: spool.filament_id,
            location_id: spool.location_id,
            colors: spool.colors.clone(),
            color_name: spool.color_name.clone(),
            initial_weight: spool.initial_weight,
            current_weight: spool.initial_weight,
            registered: Utc::now(),
            first_used: None,
            last_used: None,
            comment: spool.comment.clone(),
            archived: false,
        };
        store.spools.push(cloned.clone());
        self.flush(&store)?;
        Ok(SpoolResponse::new(cloned, filament))
    }

    // ── Location ───────────────────────────────────────────────────────────────

    pub fn list_locations(&self) -> Result<Vec<LocationResponse>> {
        let store = self.inner.read().unwrap();
        let mut items: Vec<LocationResponse> = store
            .locations
            .iter()
            .map(|loc| {
                let spool_count = store
                    .spools
                    .iter()
                    .filter(|s| s.location_id == Some(loc.id))
                    .count();
                LocationResponse {
                    location: loc.clone(),
                    spool_count,
                }
            })
            .collect();
        items.sort_by(|a, b| a.location.name.cmp(&b.location.name));
        Ok(items)
    }

    pub fn get_location(&self, id: u32) -> Result<LocationResponse> {
        let store = self.inner.read().unwrap();
        let loc = store
            .locations
            .iter()
            .find(|l| l.id == id)
            .ok_or(StoreError::NotFound)?;
        let spool_count = store
            .spools
            .iter()
            .filter(|s| s.location_id == Some(id))
            .count();
        Ok(LocationResponse {
            location: loc.clone(),
            spool_count,
        })
    }

    pub fn create_location(&self, req: CreateLocation) -> Result<LocationResponse> {
        if req.name.trim().is_empty() {
            return Err(StoreError::Validation("name must not be empty".into()));
        }
        let mut store = self.inner.write().unwrap();
        let existing: HashSet<u32> = store.locations.iter().map(|l| l.id).collect();
        let id = Self::new_id(&existing);
        let location = Location { id, name: req.name };
        store.locations.push(location.clone());
        self.flush(&store)?;
        Ok(LocationResponse { location, spool_count: 0 })
    }

    pub fn update_location(&self, id: u32, req: UpdateLocation) -> Result<LocationResponse> {
        if req.name.trim().is_empty() {
            return Err(StoreError::Validation("name must not be empty".into()));
        }
        let mut store = self.inner.write().unwrap();
        let loc = store
            .locations
            .iter_mut()
            .find(|l| l.id == id)
            .ok_or(StoreError::NotFound)?;
        loc.name = req.name;
        let loc = loc.clone();
        let spool_count = store
            .spools
            .iter()
            .filter(|s| s.location_id == Some(id))
            .count();
        self.flush(&store)?;
        Ok(LocationResponse { location: loc, spool_count })
    }

    pub fn delete_location(&self, id: u32) -> Result<()> {
        let mut store = self.inner.write().unwrap();
        let referencing: Vec<u32> = store
            .spools
            .iter()
            .filter(|s| s.location_id == Some(id))
            .map(|s| s.id)
            .collect();
        if !referencing.is_empty() {
            return Err(StoreError::Conflict(format!(
                "location is referenced by spools: {:?}",
                referencing
            )));
        }
        let before = store.locations.len();
        store.locations.retain(|l| l.id != id);
        if store.locations.len() == before {
            return Err(StoreError::NotFound);
        }
        self.flush(&store)?;
        Ok(())
    }

    // ── Helpers ────────────────────────────────────────────────────────────────

    pub fn find_materials(&self) -> Vec<String> {
        let store = self.inner.read().unwrap();
        let mut materials: Vec<String> = store
            .filaments
            .iter()
            .filter_map(|f| f.material.as_ref().map(|m| m.abbreviation().to_string()))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        materials.sort();
        materials
    }

    pub fn find_lot_numbers(&self) -> Vec<String> {
        // lot_number is not a current field; kept as a stub for future use.
        vec![]
    }

    // ── Settings ───────────────────────────────────────────────────────────────

    pub fn get_settings(&self) -> HashMap<String, String> {
        self.inner.read().unwrap().settings.clone()
    }

    pub fn put_setting(&self, key: String, value: String) -> Result<()> {
        let mut store = self.inner.write().unwrap();
        store.settings.insert(key, value);
        self.flush(&store)?;
        Ok(())
    }

    pub fn get_full_store(&self) -> DataStore {
        self.inner.read().unwrap().clone()
    }

    pub fn data_file_path(&self) -> &Path {
        &self.path
    }
}

// ── Sort / paginate helpers ────────────────────────────────────────────────────

fn sort_items<T, F>(items: &mut Vec<T>, sort: Option<&str>, order: Option<&str>, key_fn: F)
where
    F: Fn(&T, &str) -> String,
{
    let field = sort.unwrap_or("registered");
    let desc = order.map(|o| o.eq_ignore_ascii_case("desc")).unwrap_or(true);
    items.sort_by(|a, b| {
        let ka = key_fn(a, field);
        let kb = key_fn(b, field);
        if desc { kb.cmp(&ka) } else { ka.cmp(&kb) }
    });
}

fn paginate<T>(items: Vec<T>, offset: usize, limit: Option<usize>) -> Vec<T> {
    let sliced = items.into_iter().skip(offset);
    match limit {
        Some(n) => sliced.take(n).collect(),
        None => sliced.collect(),
    }
}

// ── Field-update helpers ───────────────────────────────────────────────────────

/// Apply `Some(v)` → set field; `None` → leave unchanged.
fn apply_option<T>(field: &mut Option<T>, value: Option<T>) {
    if let Some(v) = value {
        *field = Some(v);
    }
}

fn apply_option_nullable(field: &mut Option<String>, value: Option<String>) {
    // None in the request payload = don't touch; Some("") = clear
    if let Some(v) = value {
        *field = if v.is_empty() { None } else { Some(v) };
    }
}

fn apply_option_nullable_u32(field: &mut Option<u32>, value: Option<u32>) {
    if let Some(v) = value {
        *field = Some(v);
    }
}

fn apply_option_nullable_dt(
    field: &mut Option<chrono::DateTime<Utc>>,
    value: Option<chrono::DateTime<Utc>>,
) {
    if let Some(v) = value {
        *field = Some(v);
    }
}
