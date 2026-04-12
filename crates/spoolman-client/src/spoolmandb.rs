//! SpoolmanDB integration: fetch, cache, and search community filament data.
//!
//! Data source: <https://donkie.github.io/SpoolmanDB/filaments.json>
//! Cached in localStorage under `spoolmandb_cache` with a 24-hour TTL and
//! ETag-based conditional re-fetch.

use serde::{Deserialize, Serialize};
use spoolman_types::models::MaterialType;

pub const DB_URL: &str = "https://donkie.github.io/SpoolmanDB/filaments.json";
const CACHE_KEY: &str = "spoolmandb_cache";
const CACHE_TTL_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0;

/// One entry from the SpoolmanDB `filaments.json` file.
/// Optional fields use `#[serde(default)]` so missing keys don't fail deserialization.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpoolmanEntry {
    #[serde(default)]
    pub manufacturer: String,
    /// Color / variant name (e.g. "Galaxy Black").
    #[serde(default)]
    pub name: String,
    /// Material string, potentially composite (e.g. "PLA+", "PETG-CF").
    #[serde(default)]
    pub material: String,
    /// Density in g/cm³.
    #[serde(default)]
    pub density: f32,
    /// Filament diameter in mm.
    #[serde(default)]
    pub diameter: f32,
    /// Recommended extruder temperature in °C.
    #[serde(default)]
    pub extruder_temp: Option<u32>,
    /// Recommended bed temperature in °C.
    #[serde(default)]
    pub bed_temp: Option<u32>,
    /// Filament weight on the spool in grams.
    #[serde(default)]
    pub weight: Option<f32>,
    /// Empty spool weight in grams.
    #[serde(default)]
    pub spool_weight: Option<f32>,
    /// Hex color string without `#` (e.g. "ff0000").
    #[serde(default)]
    pub color_hex: Option<String>,
}

/// localStorage cache envelope.
#[derive(Serialize, Deserialize)]
struct CacheEntry {
    data: Vec<SpoolmanEntry>,
    /// HTTP ETag value from the last successful 200 response.
    etag: String,
    /// Timestamp of last successful fetch, as ms since Unix epoch.
    fetched_at: f64,
}

// ── Material parsing ───────────────────────────────────────────────────────────

/// Known dash-prefixed composite suffixes, longest first to avoid partial matches.
const DASH_SUFFIXES: &[&str] = &["-ESD", "-HF", "-CF", "-GF"];

/// Split a SpoolmanDB material string into a `(MaterialType, Option<modifier>)` pair.
///
/// Composite suffixes (`+`, `-CF`, `-GF`, `-HF`, `-ESD`) are stripped from the
/// base material before mapping via `MaterialType::from_abbreviation`. The stripped
/// suffix (without the leading `-`) becomes the modifier string.
///
/// # Examples
/// ```
/// # use spoolman_client::spoolmandb::parse_material;
/// # use spoolman_types::models::MaterialType;
/// let (mat, modi) = parse_material("PLA");
/// assert_eq!(mat, MaterialType::Pla);
/// assert_eq!(modi, None);
///
/// let (mat, modi) = parse_material("PLA+");
/// assert_eq!(mat, MaterialType::Pla);
/// assert_eq!(modi, Some("+".to_string()));
///
/// let (mat, modi) = parse_material("PETG-CF");
/// assert_eq!(mat, MaterialType::Petg);
/// assert_eq!(modi, Some("CF".to_string()));
/// ```
pub fn parse_material(s: &str) -> (MaterialType, Option<String>) {
    for suffix in DASH_SUFFIXES {
        if let Some(base) = s.strip_suffix(suffix) {
            // modifier is the suffix without the leading '-'
            let modifier = suffix.trim_start_matches('-').to_string();
            return (MaterialType::from_abbreviation(base), Some(modifier));
        }
    }
    if let Some(base) = s.strip_suffix('+') {
        return (MaterialType::from_abbreviation(base), Some("+".to_string()));
    }
    (MaterialType::from_abbreviation(s), None)
}

// ── Cache helpers (wasm32 only — uses localStorage + js_sys::Date) ────────────

#[cfg(target_arch = "wasm32")]
fn now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(target_arch = "wasm32")]
fn read_cache() -> Option<CacheEntry> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let raw = storage.get_item(CACHE_KEY).ok()??;
    serde_json::from_str(&raw).ok()
}

#[cfg(target_arch = "wasm32")]
fn write_cache(entry: &CacheEntry) {
    if let Some(Ok(Some(storage))) = web_sys::window().map(|w| w.local_storage()) {
        if let Ok(json) = serde_json::to_string(entry) {
            let _ = storage.set_item(CACHE_KEY, &json);
        }
    }
}

// ── Network fetch (wasm32 only) ────────────────────────────────────────────────

/// Stub for non-wasm targets (native tests, bin check). Never called at runtime.
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_spoolmandb() -> Result<Vec<SpoolmanEntry>, String> {
    Err("SpoolmanDB lookup requires a browser environment".to_string())
}

/// Fetch SpoolmanDB, caching in localStorage with a 24-hour TTL + ETag.
///
/// Returns `Ok(entries)` on success (from cache or network).
/// Returns `Err(message)` only when no cache exists and the fetch fails.
#[cfg(target_arch = "wasm32")]
pub async fn load_spoolmandb() -> Result<Vec<SpoolmanEntry>, String> {
    let cached = read_cache();

    // Return cached data immediately if it's fresh.
    if let Some(ref c) = cached {
        if now_ms() - c.fetched_at < CACHE_TTL_MS {
            return Ok(c.data.clone());
        }
    }

    // Build request, adding If-None-Match if we have an ETag.
    let mut req = gloo_net::http::Request::get(DB_URL);
    if let Some(ref c) = cached {
        if !c.etag.is_empty() {
            req = req.header("If-None-Match", &c.etag);
        }
    }

    match req.send().await {
        Ok(resp) if resp.status() == 304 => {
            // Not Modified — bump timestamp and return cached data.
            if let Some(mut c) = cached {
                c.fetched_at = now_ms();
                write_cache(&c);
                return Ok(c.data);
            }
            // Shouldn't happen (304 requires a prior request with ETag), but fall through.
            Err("Received 304 but no cache exists".to_string())
        }
        Ok(resp) if resp.status() == 200 => {
            let etag = resp
                .headers()
                .get("etag")
                .unwrap_or_default();
            match resp.json::<Vec<SpoolmanEntry>>().await {
                Ok(data) => {
                    write_cache(&CacheEntry {
                        data: data.clone(),
                        etag,
                        fetched_at: now_ms(),
                    });
                    Ok(data)
                }
                Err(e) => {
                    // Parse error — serve stale cache if available.
                    if let Some(c) = cached {
                        return Ok(c.data);
                    }
                    Err(format!("Failed to parse SpoolmanDB: {e}"))
                }
            }
        }
        Ok(resp) => {
            // Non-200/304 status.
            if let Some(c) = cached {
                return Ok(c.data);
            }
            Err(format!("SpoolmanDB returned HTTP {}", resp.status()))
        }
        Err(e) => {
            // Network error — serve stale cache if available.
            if let Some(c) = cached {
                return Ok(c.data);
            }
            Err(format!("Failed to fetch SpoolmanDB: {e}"))
        }
    }
}

// ── Unit tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use spoolman_types::models::MaterialType;

    #[test]
    fn plain_material() {
        let (mat, modi) = parse_material("PLA");
        assert_eq!(mat, MaterialType::Pla);
        assert_eq!(modi, None);
    }

    #[test]
    fn plus_suffix() {
        let (mat, modi) = parse_material("PLA+");
        assert_eq!(mat, MaterialType::Pla);
        assert_eq!(modi, Some("+".to_string()));
    }

    #[test]
    fn cf_suffix() {
        let (mat, modi) = parse_material("PETG-CF");
        assert_eq!(mat, MaterialType::Petg);
        assert_eq!(modi, Some("CF".to_string()));
    }

    #[test]
    fn gf_suffix() {
        let (mat, modi) = parse_material("ABS-GF");
        assert_eq!(mat, MaterialType::Abs);
        assert_eq!(modi, Some("GF".to_string()));
    }

    #[test]
    fn unknown_base() {
        let (mat, modi) = parse_material("NYLON-CF");
        assert_eq!(mat, MaterialType::Other("NYLON".to_string()));
        assert_eq!(modi, Some("CF".to_string()));
    }

    #[test]
    fn fully_unknown() {
        let (mat, modi) = parse_material("WOODFILL");
        assert_eq!(mat, MaterialType::Other("WOODFILL".to_string()));
        assert_eq!(modi, None);
    }
}
