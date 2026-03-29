//! Typed API client for the spoolman-server REST API.
//!
//! All functions are async and return `Result<T, ApiError>`.
//! Base URL defaults to `/` (relative), so the WASM frontend works
//! whether served on port 8000 or via cargo-leptos dev proxy.

use gloo_net::http::Request;
use spoolman_types::{
    requests::PutSetting,
    responses::{InfoResponse, SpoolmanDbEntry},
};

pub mod filament;
pub mod location;
pub mod spool;

pub use filament::*;
pub use location::*;
pub use spool::*;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP {}: {}", self.status, self.message)
    }
}

// ── Generic helpers ───────────────────────────────────────────────────────────

pub async fn get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    let resp = Request::get(path).send().await.map_err(|e| ApiError {
        status: 0,
        message: e.to_string(),
    })?;
    if resp.ok() {
        resp.json().await.map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })
    } else {
        Err(ApiError {
            status: resp.status(),
            message: resp.status_text().to_string(),
        })
    }
}

pub async fn post_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let resp = Request::post(path)
        .json(body)
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?;
    if resp.ok() {
        resp.json().await.map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })
    } else {
        Err(ApiError {
            status: resp.status(),
            message: resp.status_text().to_string(),
        })
    }
}

pub async fn patch_json<B: serde::Serialize, T: serde::de::DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let resp = Request::patch(path)
        .json(body)
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?;
    if resp.ok() {
        resp.json().await.map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })
    } else {
        Err(ApiError {
            status: resp.status(),
            message: resp.status_text().to_string(),
        })
    }
}

pub async fn delete(path: &str) -> Result<(), ApiError> {
    let resp = Request::delete(path).send().await.map_err(|e| ApiError {
        status: 0,
        message: e.to_string(),
    })?;
    if resp.ok() || resp.status() == 204 {
        Ok(())
    } else {
        Err(ApiError {
            status: resp.status(),
            message: resp.status_text().to_string(),
        })
    }
}

// ── Info ──────────────────────────────────────────────────────────────────────

pub async fn fetch_info() -> Result<InfoResponse, ApiError> {
    get("/api/v1/info").await
}

// ── Settings ──────────────────────────────────────────────────────────────────

pub async fn fetch_settings() -> Result<std::collections::HashMap<String, String>, ApiError> {
    get("/api/v1/setting").await
}

pub async fn put_setting(key: &str, value: String) -> Result<(), ApiError> {
    let body = PutSetting { value };
    let resp = Request::put(&format!("/api/v1/setting/{key}"))
        .json(&body)
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?
        .send()
        .await
        .map_err(|e| ApiError {
            status: 0,
            message: e.to_string(),
        })?;
    if resp.ok() || resp.status() == 204 {
        Ok(())
    } else {
        Err(ApiError {
            status: resp.status(),
            message: resp.status_text().to_string(),
        })
    }
}

// ── SpoolmanDB search proxy ────────────────────────────────────────────────────

pub async fn search_spoolmandb(q: &str) -> Result<Vec<SpoolmanDbEntry>, ApiError> {
    get(&format!(
        "/api/v1/filament/search?q={}",
        urlencoding_encode(q)
    ))
    .await
}

fn urlencoding_encode(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
                vec![c]
            } else {
                format!("%{:02X}", c as u32).chars().collect()
            }
        })
        .collect()
}
