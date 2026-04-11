use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn router() -> Router<crate::store::JsonStore> {
    Router::new()
        .route("/health", get(health))
        .route("/info", get(info))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn info(
    axum::extract::State(store): axum::extract::State<crate::store::JsonStore>,
) -> Json<spoolman_types::responses::InfoResponse> {
    let data_directory = store
        .data_file_path()
        .parent()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    Json(spoolman_types::responses::InfoResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        debug: store.debug_mode(),
        data_directory,
        automatic_backup: store.automatic_backup(),
    })
}
