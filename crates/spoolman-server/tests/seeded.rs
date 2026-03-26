/// Tests that create operations succeed when the store is pre-populated with
/// the assets/spoolman.json test fixture (B12 regression test).
mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;

#[tokio::test]
async fn create_location_with_seeded_data_returns_201() {
    let (app, _dir) = common::make_app_with_test_data();
    let (status, body) =
        common::request(&app, Method::POST, "/api/v1/location", Some(json!({ "name": "New Shelf" }))).await;
    assert_eq!(status, StatusCode::CREATED, "body: {body}");
}

#[tokio::test]
async fn create_filament_with_seeded_data_returns_201() {
    let (app, _dir) = common::make_app_with_test_data();
    let (status, body) =
        common::request(&app, Method::POST, "/api/v1/filament", Some(json!({ "density": 1.24 }))).await;
    assert_eq!(status, StatusCode::CREATED, "body: {body}");
}

#[tokio::test]
async fn create_spool_with_seeded_data_returns_201() {
    let (app, _dir) = common::make_app_with_test_data();
    // Filament 1001 (Prusament PLA) is present in the test fixture.
    let (status, body) = common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(json!({ "filament_id": 1001, "colors": [], "initial_weight": 1000.0 })),
    ).await;
    assert_eq!(status, StatusCode::CREATED, "body: {body}");
}
