mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;

fn minimal_filament() -> serde_json::Value {
    json!({ "density": 1.24 })
}

#[tokio::test]
async fn create_filament_returns_201() {
    let (app, _dir) = common::make_app();
    let (status, body) = common::request(&app, Method::POST, "/api/v1/filament", Some(minimal_filament())).await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].as_u64().unwrap() > 0);
    assert!((body["density"].as_f64().unwrap() - 1.24).abs() < 0.01);
}

#[tokio::test]
async fn get_filament_returns_200() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(&app, Method::POST, "/api/v1/filament", Some(minimal_filament())).await;
    let id = created["id"].as_u64().unwrap();

    let (status, body) = common::request(&app, Method::GET, &format!("/api/v1/filament/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"].as_u64().unwrap(), id);
}

#[tokio::test]
async fn get_unknown_filament_returns_404() {
    let (app, _dir) = common::make_app();
    let (status, _) = common::request(&app, Method::GET, "/api/v1/filament/999999", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_filaments_returns_all() {
    let (app, _dir) = common::make_app();
    common::request(&app, Method::POST, "/api/v1/filament", Some(json!({ "density": 1.24 }))).await;
    common::request(&app, Method::POST, "/api/v1/filament", Some(json!({ "density": 1.27 }))).await;

    let (status, body) = common::request(&app, Method::GET, "/api/v1/filament", None).await;
    assert_eq!(status, StatusCode::OK);
    let items = body.as_array().unwrap();
    assert_eq!(items.len(), 2);
}

#[tokio::test]
async fn update_filament_returns_200() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(&app, Method::POST, "/api/v1/filament", Some(minimal_filament())).await;
    let id = created["id"].as_u64().unwrap();

    let (status, body) = common::request(
        &app,
        Method::PATCH,
        &format!("/api/v1/filament/{id}"),
        Some(json!({ "manufacturer": "Bambu Lab" })),
    ).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["manufacturer"], "Bambu Lab");
}

#[tokio::test]
async fn delete_filament_returns_204_and_subsequent_get_404() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(&app, Method::POST, "/api/v1/filament", Some(minimal_filament())).await;
    let id = created["id"].as_u64().unwrap();

    let (status, _) = common::request(&app, Method::DELETE, &format!("/api/v1/filament/{id}"), None).await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, _) = common::request(&app, Method::GET, &format!("/api/v1/filament/{id}"), None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn delete_filament_blocked_by_spool_returns_409() {
    let (app, _dir) = common::make_app();
    let (_, filament) = common::request(&app, Method::POST, "/api/v1/filament", Some(minimal_filament())).await;
    let fid = filament["id"].as_u64().unwrap();

    // Create a spool that references the filament
    common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(json!({ "filament_id": fid, "colors": [], "initial_weight": 1000.0 })),
    ).await;

    let (status, _) = common::request(&app, Method::DELETE, &format!("/api/v1/filament/{fid}"), None).await;
    assert_eq!(status, StatusCode::CONFLICT);
}
