mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;

async fn create_filament(app: &axum::Router) -> u64 {
    let (_, body) = common::request(
        app,
        Method::POST,
        "/api/v1/filament",
        Some(json!({ "density": 1.24 })),
    )
    .await;
    body["id"].as_u64().unwrap()
}

fn spool_body(filament_id: u64) -> serde_json::Value {
    json!({ "filament_id": filament_id, "colors": [], "initial_weight": 1000.0 })
}

#[tokio::test]
async fn create_spool_returns_201_with_nested_filament() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;

    let (status, body) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].as_u64().unwrap() > 0);
    assert_eq!(body["filament_id"].as_u64().unwrap(), fid);
    assert!(body["filament"]["id"].as_u64().is_some());
}

#[tokio::test]
async fn create_spool_with_unknown_filament_returns_404() {
    let (app, _dir) = common::make_app();
    let (status, _) = common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(spool_body(999999)),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn get_spool_returns_200() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    let (_, created) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    let sid = created["id"].as_u64().unwrap();

    let (status, body) =
        common::request(&app, Method::GET, &format!("/api/v1/spool/{sid}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"].as_u64().unwrap(), sid);
}

#[tokio::test]
async fn get_unknown_spool_returns_404() {
    let (app, _dir) = common::make_app();
    let (status, _) = common::request(&app, Method::GET, "/api/v1/spool/999999", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_spools_returns_all() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;

    let (status, body) = common::request(&app, Method::GET, "/api/v1/spool", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
async fn update_spool_weight_sets_last_used() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    let (_, created) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    let sid = created["id"].as_u64().unwrap();

    let (status, body) = common::request(
        &app,
        Method::PATCH,
        &format!("/api/v1/spool/{sid}"),
        Some(json!({ "current_weight": 800.0 })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!((body["current_weight"].as_f64().unwrap() - 800.0).abs() < 0.01);
    assert!(!body["last_used"].is_null());
}

#[tokio::test]
async fn clone_spool_returns_201_with_different_id() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    let (_, created) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    let sid = created["id"].as_u64().unwrap();

    let (status, body) = common::request(
        &app,
        Method::POST,
        &format!("/api/v1/spool/{sid}/clone"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_ne!(body["id"].as_u64().unwrap(), sid);
    assert_eq!(body["filament_id"].as_u64().unwrap(), fid);
}

/// Regression test for B12 / B8: creating a spool whose filament has
/// net_weight=0 previously produced NaN in remaining_pct, causing serde_json
/// to fail and Axum to return HTTP 500 instead of 201.
#[tokio::test]
async fn create_spool_with_zero_net_weight_filament_returns_201() {
    let (app, _dir) = common::make_app();
    let (_, fil) = common::request(
        &app,
        Method::POST,
        "/api/v1/filament",
        Some(json!({ "density": 1.24, "net_weight": 0.0 })),
    )
    .await;
    let fid = fil["id"].as_u64().unwrap();

    let (status, body) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    assert_eq!(status, StatusCode::CREATED, "body: {body}");
    assert!(
        body["remaining_pct"].is_null(),
        "remaining_pct should be None when net_weight=0"
    );
}

/// Task 6.2: creating a spool with a price returns `price_per_gram` in the response.
/// Also verifies that a spool without price returns `null` for `price_per_gram`.
#[tokio::test]
async fn spool_price_returns_price_per_gram() {
    let (app, _dir) = common::make_app();
    // Create filament with a known net_weight so price_per_gram is deterministic
    let (_, fil) = common::request(
        &app,
        Method::POST,
        "/api/v1/filament",
        Some(json!({ "density": 1.24 })),
    )
    .await;
    let fid = fil["id"].as_u64().unwrap();

    // Spool with price=20.0 and net_weight=1000.0 → price_per_gram = 0.02
    let (status, body) = common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(json!({ "filament_id": fid, "colors": [], "initial_weight": 1200.0, "net_weight": 1000.0, "price": 20.0 })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "body: {body}");
    let ppg = body["price_per_gram"].as_f64().expect("price_per_gram should be present");
    assert!((ppg - 0.02).abs() < 0.0001, "expected ~0.02, got {ppg}");

    // Spool without price → price_per_gram must be null
    let (status2, body2) = common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(json!({ "filament_id": fid, "colors": [], "initial_weight": 1200.0, "net_weight": 1000.0 })),
    )
    .await;
    assert_eq!(status2, StatusCode::CREATED, "body: {body2}");
    assert!(body2["price_per_gram"].is_null(), "price_per_gram should be null when no price set");
}

#[tokio::test]
async fn delete_spool_returns_204_and_subsequent_get_404() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    let (_, created) =
        common::request(&app, Method::POST, "/api/v1/spool", Some(spool_body(fid))).await;
    let sid = created["id"].as_u64().unwrap();

    let (status, _) =
        common::request(&app, Method::DELETE, &format!("/api/v1/spool/{sid}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) =
        common::request(&app, Method::GET, &format!("/api/v1/spool/{sid}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
