//! filamentcolors.xyz integration: fetch, cache, and search measured swatch data.
//!
//! Data source: <https://filamentcolors.xyz/api/swatch/> (public, CORS-open, anonymous
//! API throttled 100/min & 3600/hour per IP). Cached in localStorage under
//! `filamentcolors_cache` with a 24-hour TTL, mirroring the SpoolmanDB cache.

use serde::{Deserialize, Serialize};

pub const SWATCH_LIST_URL: &str = "https://filamentcolors.xyz/api/swatch/?page_size=100";
#[cfg(target_arch = "wasm32")]
const CACHE_KEY: &str = "filamentcolors_cache";
#[cfg(target_arch = "wasm32")]
const CACHE_TTL_MS: f64 = 24.0 * 60.0 * 60.0 * 1000.0;
/// Safety cap on paginated fetches (2258 entries / 100 per page is ~23 pages today).
#[cfg(target_arch = "wasm32")]
const MAX_PAGES: u32 = 60;

/// A filamentcolors.xyz manufacturer reference (only the fields we use; the
/// API response carries more, which serde ignores).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilamentColorsManufacturer {
    pub name: String,
}

/// One swatch entry from the filamentcolors.xyz `SwatchViewSet` list.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilamentColorsSwatch {
    pub id: u32,
    pub manufacturer: FilamentColorsManufacturer,
    pub color_name: String,
    /// Measured hex color, without a leading `#` (e.g. "E9EDED").
    pub hex_color: String,
    /// Thumbnail/card image URL for this swatch, if published.
    pub card_img: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Deserialize)]
struct SwatchPage {
    next: Option<String>,
    results: Vec<FilamentColorsSwatch>,
}

/// localStorage cache envelope.
#[cfg(target_arch = "wasm32")]
#[derive(Serialize, Deserialize)]
struct CacheEntry {
    data: Vec<FilamentColorsSwatch>,
    fetched_at: f64,
}

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

/// Stub for non-wasm targets (native tests, bin check). Never called at runtime.
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_filamentcolors() -> Result<Vec<FilamentColorsSwatch>, String> {
    Err("filamentcolors.xyz lookup requires a browser environment".to_string())
}

/// Fetch the full filamentcolors.xyz swatch list, paginating through all
/// pages, caching the flattened result in localStorage with a 24-hour TTL.
///
/// Returns `Ok(entries)` on success (from cache or network).
/// Returns `Err(message)` only when no cache exists and the fetch fails.
#[cfg(target_arch = "wasm32")]
pub async fn load_filamentcolors() -> Result<Vec<FilamentColorsSwatch>, String> {
    let cached = read_cache();
    if let Some(ref c) = cached {
        if now_ms() - c.fetched_at < CACHE_TTL_MS {
            return Ok(c.data.clone());
        }
    }

    match fetch_all_pages().await {
        Ok(data) => {
            write_cache(&CacheEntry {
                data: data.clone(),
                fetched_at: now_ms(),
            });
            Ok(data)
        }
        Err(e) => {
            if let Some(c) = cached {
                Ok(c.data)
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
async fn fetch_all_pages() -> Result<Vec<FilamentColorsSwatch>, String> {
    let mut url = SWATCH_LIST_URL.to_string();
    let mut all = Vec::new();
    for _ in 0..MAX_PAGES {
        let resp = gloo_net::http::Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch filamentcolors.xyz: {e}"))?;
        if !resp.ok() {
            return Err(format!("filamentcolors.xyz returned HTTP {}", resp.status()));
        }
        let page: SwatchPage = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse filamentcolors.xyz response: {e}"))?;
        all.extend(page.results);
        match page.next {
            Some(next) => url = next,
            None => break,
        }
    }
    Ok(all)
}
