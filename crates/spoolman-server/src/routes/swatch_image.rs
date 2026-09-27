use axum::{
    body::Bytes,
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    path::PathBuf,
};

use crate::{
    routes::error::Result,
    store::{JsonStore, StoreError},
};

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/", post(store_image))
        .route("/{id}", get(serve_image))
}

#[derive(Deserialize)]
struct StoreImageRequest {
    url: String,
}

#[derive(Serialize)]
struct StoreImageResponse {
    id: String,
}

fn swatch_images_dir(store: &JsonStore) -> PathBuf {
    store
        .get_data_file_path()
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_default()
        .join("swatch_images")
}

/// Stable, content-addressed filename base for a source URL. Never derived
/// from anything else client-supplied, so it is safe to join onto the
/// storage directory without a traversal check.
fn hash_url(url: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    url.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn guess_extension(url: &str, content_type: Option<&str>) -> &'static str {
    let from_url = url.rsplit('.').next().unwrap_or("");
    match from_url.to_lowercase().as_str() {
        "png" => return "png",
        "jpg" | "jpeg" => return "jpg",
        "webp" => return "webp",
        "gif" => return "gif",
        _ => {}
    }
    match content_type.unwrap_or("") {
        ct if ct.contains("png") => "png",
        ct if ct.contains("webp") => "webp",
        ct if ct.contains("gif") => "gif",
        _ => "jpg",
    }
}

async fn store_image(
    State(store): State<JsonStore>,
    Json(body): Json<StoreImageRequest>,
) -> Result<Json<StoreImageResponse>> {
    let dir = swatch_images_dir(&store);
    std::fs::create_dir_all(&dir).map_err(StoreError::from)?;

    let id_base = hash_url(&body.url);
    // Dedupe: an existing file with this hash was already downloaded from this URL.
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            if entry.file_name().to_string_lossy().starts_with(&id_base) {
                return Ok(Json(StoreImageResponse {
                    id: entry.file_name().to_string_lossy().into_owned(),
                }));
            }
        }
    }

    let resp = reqwest::get(&body.url)
        .await
        .map_err(|e| StoreError::Validation(format!("failed to download image: {e}")))?;
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
    let ext = guess_extension(&body.url, content_type.as_deref());
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| StoreError::Validation(format!("failed to read image body: {e}")))?;

    let filename = format!("{id_base}.{ext}");
    // `filename` is server-generated (hash + fixed extension), not user input.
    std::fs::write(dir.join(&filename), &bytes).map_err(StoreError::from)?; // nosemgrep: path-traversal

    Ok(Json(StoreImageResponse { id: filename }))
}

async fn serve_image(State(store): State<JsonStore>, Path(id): Path<String>) -> Result<Response> {
    // Reject anything that isn't a bare filename we could have generated ourselves.
    if id.contains('/') || id.contains('\\') || id.contains("..") {
        return Err(StoreError::NotFound.into());
    }
    let path = swatch_images_dir(&store).join(&id);
    if !path.exists() {
        return Err(StoreError::NotFound.into());
    }
    let bytes = std::fs::read(&path).map_err(StoreError::from)?; // nosemgrep: path-traversal
    let content_type = match id.rsplit('.').next().unwrap_or("") {
        "png" => "image/png",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/jpeg",
    };
    Ok(([(header::CONTENT_TYPE, content_type)], Bytes::from(bytes)).into_response())
}
