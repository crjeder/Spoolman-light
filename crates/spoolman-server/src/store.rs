use chrono::Utc;
use rand::RngExt;
use spoolman_types::{
    models::{DataStore, Filament, Location, Spool, StoreMeta},
    requests::{
        CreateFilament, CreateLocation, CreateSpool, UpdateFilament, UpdateLocation, UpdateSpool,
    },
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

pub struct SpoolFilter<'a> {
    pub filament_id: Option<u32>,
    pub location_id: Option<u32>,
    pub allow_archived: bool,
    pub sort: Option<&'a str>,
    pub order: Option<&'a str>,
    pub offset: usize,
    pub limit: Option<usize>,
}

/// Thread-safe JSON-backed data store.
#[derive(Clone)]
pub struct JsonStore {
    inner: Arc<RwLock<DataStore>>,
    path: PathBuf,
    automatic_backup: bool,
    debug_mode: bool,
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
        let store = Self {
            inner: Arc::new(RwLock::new(DataStore::default())),
            path: resolved,
            automatic_backup: true,
            debug_mode: false,
        };
        let data = store.read_and_migrate()?;
        *store.inner.write().unwrap() = data;
        Ok(store)
    }

    /// Re-read the data file from disk and replace the in-memory store.
    ///
    /// On any read/parse error the in-memory store is left untouched.
    pub fn reload(&self) -> Result<()> {
        let data = self.read_and_migrate()?;
        *self.inner.write().unwrap() = data;
        Ok(())
    }

    /// Replace the entire store with `data` (e.g. from a previously exported
    /// backup), migrating it to the current schema, flushing to disk, and
    /// then swapping the in-memory copy.
    ///
    /// Rejects a `data.meta.schema_version` newer than the current schema
    /// version. On any error the on-disk file and in-memory store are left
    /// untouched.
    pub fn import(&self, mut data: DataStore) -> Result<()> {
        let current_version = StoreMeta::default().schema_version;
        if data.meta.schema_version > current_version {
            return Err(StoreError::Validation(format!(
                "unsupported schema_version {} (server supports up to {})",
                data.meta.schema_version, current_version
            )));
        }
        self.migrate(&mut data)?;
        self.flush(&data)?;
        *self.inner.write().unwrap() = data;
        Ok(())
    }

    /// Read `self.path` from disk (or default if it doesn't exist yet) and
    /// run any pending schema migrations.
    fn read_and_migrate(&self) -> Result<DataStore> {
        let mut data: DataStore = if self.path.exists() {
            // `self.path` is the canonicalized output of resolve_data_path; the
            // original path is operator-configured (env var), not user input.
            let contents = std::fs::read_to_string(&self.path)?; // nosemgrep: path-traversal
            serde_json::from_str(&contents)?
        } else {
            DataStore::default()
        };
        // Run any pending schema migrations before exposing the store.
        self.migrate(&mut data)?;
        Ok(data)
    }

    /// Configure runtime flags from the server config.
    pub fn with_config(mut self, automatic_backup: bool, debug_mode: bool) -> Self {
        self.automatic_backup = automatic_backup;
        self.debug_mode = debug_mode;
        self
    }

    /// Migrate the store to the current schema version if needed.
    ///
    /// v1 → v2: move `net_weight` from each `Filament` onto its referencing `Spool`s.
    fn migrate(&self, data: &mut DataStore) -> Result<()> {
        if data.meta.schema_version < 2 {
            tracing::info!("migrating data store from schema v1 to v2 (net_weight → spool)");
            let filament_nw: HashMap<u32, Option<f32>> = data
                .filaments
                .iter()
                .map(|f| (f.id, f.net_weight))
                .collect();
            for spool in &mut data.spools {
                if spool.net_weight.is_none() {
                    spool.net_weight = filament_nw.get(&spool.filament_id).copied().flatten();
                }
            }
            for filament in &mut data.filaments {
                filament.net_weight = None;
            }
            data.meta.schema_version = 2;
            self.flush(data)?;
            tracing::info!("migration to schema v2 complete");
        }
        Ok(())
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
        let mut rng = rand::rng();
        loop {
            let id: u32 = rng.random_range(1..=u32::MAX);
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
            "id" => format!("{:010}", f.id),
            "manufacturer" => f.manufacturer.as_deref().unwrap_or("").to_string(),
            "material" => f
                .material
                .as_ref()
                .map(|m| m.abbreviation())
                .unwrap_or("")
                .to_string(),
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
            net_weight: None,
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
        if let Some(v) = req.diameter {
            filament.diameter = v;
        }
        if let Some(v) = req.density {
            filament.density = v;
        }
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
        check_no_referencing_spools(&store.spools, "filament", |s| s.filament_id == id)?;
        retain_or_not_found(&mut store.filaments, |f| f.id != id)?;
        self.flush(&store)
    }

    // ── Spool ──────────────────────────────────────────────────────────────────

    pub fn list_spools(&self, filter: SpoolFilter<'_>) -> Result<(Vec<SpoolResponse>, usize)> {
        let store = self.inner.read().unwrap();
        let filament_map: HashMap<u32, Filament> =
            store.filaments.iter().map(|f| (f.id, f.clone())).collect();

        let mut items: Vec<SpoolResponse> = store
            .spools
            .iter()
            .filter(|s| {
                if !filter.allow_archived && s.archived {
                    return false;
                }
                if let Some(fid) = filter.filament_id {
                    if s.filament_id != fid {
                        return false;
                    }
                }
                if let Some(lid) = filter.location_id {
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

        sort_items(&mut items, filter.sort, filter.order, |s, field| match field {
            "registered" => s.spool.registered.to_rfc3339(),
            "last_used" => s
                .spool
                .last_used
                .map(|d| d.to_rfc3339())
                .unwrap_or_default(),
            _ => s.spool.registered.to_rfc3339(),
        });

        let total = items.len();
        let items = paginate(items, filter.offset, filter.limit);
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
        // A spool must be assigned to an existing storage location.
        let location_id = req
            .location_id
            .ok_or_else(|| StoreError::Validation("location_id is required".into()))?;
        if !store.locations.iter().any(|l| l.id == location_id) {
            return Err(StoreError::Validation("location_id does not exist".into()));
        }
        if req.colors.len() > 4 {
            return Err(StoreError::Validation("a spool may have at most 4 colors".into()));
        }
        let existing: HashSet<u32> = store.spools.iter().map(|s| s.id).collect();
        let id = Self::new_id(&existing);
        let spool = Spool {
            id,
            filament_id: req.filament_id,
            location_id: Some(location_id),
            colors: req.colors,
            color_name: req.color_name,
            initial_weight: req.initial_weight,
            current_weight: req.initial_weight,
            net_weight: req.net_weight,
            price: req.price,
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
        if let Some(lid) = req.location_id {
            if !store.locations.iter().any(|l| l.id == lid) {
                return Err(StoreError::Validation("location_id does not exist".into()));
            }
        }
        if req.colors.as_ref().is_some_and(|c| c.len() > 4) {
            return Err(StoreError::Validation("a spool may have at most 4 colors".into()));
        }
        let spool = store
            .spools
            .iter_mut()
            .find(|s| s.id == id)
            .ok_or(StoreError::NotFound)?;

        let weight_changed = req.current_weight.is_some();
        if let Some(colors) = req.colors {
            spool.colors = colors;
        }
        apply_option_nullable(&mut spool.color_name, req.color_name);
        apply_option_nullable_u32(&mut spool.location_id, req.location_id);
        if let Some(w) = req.current_weight {
            spool.current_weight = w;
        }
        if let Some(nw) = req.net_weight {
            spool.net_weight = Some(nw);
        }
        apply_option(&mut spool.price, req.price);
        apply_option_nullable_dt(&mut spool.first_used, req.first_used);
        apply_option_nullable_dt(&mut spool.last_used, req.last_used);
        apply_option_nullable(&mut spool.comment, req.comment);
        if let Some(archived) = req.archived {
            spool.archived = archived;
        }
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
            net_weight: spool.net_weight,
            price: spool.price,
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
        Ok(LocationResponse {
            location,
            spool_count: 0,
        })
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
        Ok(LocationResponse {
            location: loc,
            spool_count,
        })
    }

    pub fn delete_location(&self, id: u32) -> Result<()> {
        let mut store = self.inner.write().unwrap();
        check_no_referencing_spools(&store.spools, "location", |s| s.location_id == Some(id))?;
        retain_or_not_found(&mut store.locations, |l| l.id != id)?;
        self.flush(&store)
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

    pub fn get_data_file_path(&self) -> &PathBuf {
        &self.path
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

    pub fn automatic_backup(&self) -> bool {
        self.automatic_backup
    }

    pub fn debug_mode(&self) -> bool {
        self.debug_mode
    }
}

// ── Sort / paginate helpers ────────────────────────────────────────────────────

fn check_no_referencing_spools(
    spools: &[Spool],
    entity: &str,
    predicate: impl Fn(&Spool) -> bool,
) -> Result<()> {
    let refs: Vec<u32> = spools
        .iter()
        .filter(|s| predicate(s))
        .map(|s| s.id)
        .collect();
    if refs.is_empty() {
        Ok(())
    } else {
        Err(StoreError::Conflict(format!(
            "{entity} is referenced by spools: {refs:?}"
        )))
    }
}

fn retain_or_not_found<T>(items: &mut Vec<T>, predicate: impl Fn(&T) -> bool) -> Result<()> {
    let before = items.len();
    items.retain(|x| predicate(x));
    if items.len() == before {
        Err(StoreError::NotFound)
    } else {
        Ok(())
    }
}

fn sort_items<T, F>(items: &mut [T], sort: Option<&str>, order: Option<&str>, key_fn: F)
where
    F: Fn(&T, &str) -> String,
{
    let field = sort.unwrap_or("registered");
    let desc = order
        .map(|o| o.eq_ignore_ascii_case("desc"))
        .unwrap_or(true);
    items.sort_by(|a, b| {
        let ka = key_fn(a, field);
        let kb = key_fn(b, field);
        if desc {
            kb.cmp(&ka)
        } else {
            ka.cmp(&kb)
        }
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
