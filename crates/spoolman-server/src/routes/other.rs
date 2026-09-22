use axum::{
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
    routing::{get, post, put},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{
    routes::error::Result,
    store::{JsonStore, StoreError},
};
use spoolman_types::{models::DataStore, requests::PutSetting};

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/info", get(info))
        .route("/material", get(list_materials))
        .route("/export", get(export))
        .route("/import", post(import))
        .route("/setting", get(list_settings))
        .route("/setting/{key}", put(put_setting))
        .route("/reload", post(reload))
        .route("/database/download", get(download_database))
}

async fn info(State(store): State<JsonStore>) -> Json<Value> {
    let data_directory = store
        .data_file_path()
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "debug": store.debug_mode(),
        "data_directory": data_directory,
        "automatic_backup": store.automatic_backup(),
    }))
}

async fn list_materials(State(store): State<JsonStore>) -> Json<Vec<String>> {
    Json(store.find_materials())
}

async fn export(State(store): State<JsonStore>) -> Json<Value> {
    let ds = store.get_full_store();
    Json(serde_json::to_value(ds).unwrap_or_default())
}

async fn import(
    State(store): State<JsonStore>,
    Json(body): Json<DataStore>,
) -> Result<axum::http::StatusCode> {
    store.import(body)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn list_settings(
    State(store): State<JsonStore>,
) -> Json<std::collections::HashMap<String, String>> {
    Json(store.get_settings())
}

async fn put_setting(
    State(store): State<JsonStore>,
    Path(key): Path<String>,
    Json(body): Json<PutSetting>,
) -> Result<axum::http::StatusCode> {
    store.put_setting(key, body.value)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn reload(State(store): State<JsonStore>) -> Result<axum::http::StatusCode> {
    store.reload()?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

async fn download_database(State(store): State<JsonStore>) -> Result<Response> {
    let path = store.data_file_path();
    if !path.exists() {
        return Err(StoreError::NotFound.into());
    }
    // `path` is the store's own canonicalized path, not user input.
    let contents = std::fs::read(path).map_err(StoreError::from)?; // nosemgrep: path-traversal
    Ok((
        [
            (header::CONTENT_TYPE, "application/json"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"spoolman.json\"",
            ),
        ],
        contents,
    )
        .into_response())
}
