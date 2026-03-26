use axum::{Router, body::Body, http::{Method, Request, StatusCode}};
use http_body_util::BodyExt;
use spoolman_server::{config::Config, routes, store::JsonStore};
use std::path::PathBuf;
use tempfile::TempDir;
use tower::ServiceExt;

/// Creates an isolated router backed by the seeded `assets/spoolman.json` test data.
/// Simulates the Docker test environment where the server starts with pre-existing data.
pub fn make_app_with_test_data() -> (Router, TempDir) {
    let dir = TempDir::new().unwrap();
    let data_path = dir.path().join("data.json");
    // Copy the test fixture into the writable temp dir.
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/spoolman.json");
    std::fs::copy(&fixture, &data_path).expect("failed to copy test fixture");
    let store = JsonStore::load(&data_path).unwrap();
    let cfg = Config {
        data_file: data_path,
        host: "127.0.0.1".to_string(),
        port: 0,
        base_path: String::new(),
        debug_mode: false,
        logging_level: "error".to_string(),
        cors_origin: None,
        automatic_backup: false,
        version: "test".to_string(),
        site_root: PathBuf::from("/nonexistent-test-site"),
    };
    let router = routes::build_router(store, &cfg);
    (router, dir)
}

/// Creates an isolated router backed by an empty in-memory store in a temp dir.
/// The returned `TempDir` must be kept alive for the duration of the test.
pub fn make_app() -> (Router, TempDir) {
    let dir = TempDir::new().unwrap();
    // Use a path that doesn't exist yet so JsonStore creates an empty default store.
    let data_path = dir.path().join("data.json");
    let store = JsonStore::load(&data_path).unwrap();
    let cfg = Config {
        data_file: data_path,
        host: "127.0.0.1".to_string(),
        port: 0,
        base_path: String::new(),
        debug_mode: false,
        logging_level: "error".to_string(),
        cors_origin: None,
        automatic_backup: false,
        version: "test".to_string(),
        site_root: PathBuf::from("/nonexistent-test-site"),
    };
    let router = routes::build_router(store, &cfg);
    (router, dir)
}

/// Dispatch a single request to the app and return (status, parsed JSON body).
/// Passes `Accept-Encoding: identity` to avoid compressed responses.
pub async fn request(
    app: &Router,
    method: Method,
    path: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, serde_json::Value) {
    let body_bytes = match &body {
        Some(v) => Body::from(serde_json::to_vec(v).unwrap()),
        None => Body::empty(),
    };

    let req = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .header("accept-encoding", "identity")
        .body(body_bytes)
        .unwrap();

    let resp = app.clone().oneshot(req).await.unwrap();
    let status = resp.status();
    let bytes = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = if bytes.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
    };
    (status, json)
}
