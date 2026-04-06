use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use serde_json::{json, Value};

use crate::{routes::error::Result, store::JsonStore};
use spoolman_types::requests::PutSetting;

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/info", get(info))
        .route("/material", get(list_materials))
        .route("/export", get(export))
        .route("/setting", get(list_settings))
        .route("/setting/{key}", put(put_setting))
}

async fn info(State(store): State<JsonStore>) -> Json<Value> {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "data_file": store.get_data_file_path().to_string_lossy(),
    }))
}

async fn list_materials(State(store): State<JsonStore>) -> Json<Vec<String>> {
    Json(store.find_materials())
}

async fn export(State(store): State<JsonStore>) -> Json<Value> {
    let ds = store.get_full_store();
    Json(serde_json::to_value(ds).unwrap_or_default())
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
