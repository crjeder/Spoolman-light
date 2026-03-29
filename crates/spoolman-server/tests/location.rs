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

#[tokio::test]
async fn create_location_returns_201() {
    let (app, _dir) = common::make_app();
    let (status, body) = common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "Shelf A" })),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].as_u64().unwrap() > 0);
    assert_eq!(body["name"], "Shelf A");
    assert_eq!(body["spool_count"].as_u64().unwrap(), 0);
}

#[tokio::test]
async fn get_location_returns_200_with_spool_count() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "Shelf B" })),
    )
    .await;
    let lid = created["id"].as_u64().unwrap();

    let (status, body) =
        common::request(&app, Method::GET, &format!("/api/v1/location/{lid}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"].as_u64().unwrap(), lid);
    assert!(body.get("spool_count").is_some());
}

#[tokio::test]
async fn get_unknown_location_returns_404() {
    let (app, _dir) = common::make_app();
    let (status, _) = common::request(&app, Method::GET, "/api/v1/location/999999", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn list_locations_returns_created() {
    let (app, _dir) = common::make_app();
    common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "Drawer 1" })),
    )
    .await;

    let (status, body) = common::request(&app, Method::GET, "/api/v1/location", None).await;
    assert_eq!(status, StatusCode::OK);
    let items = body.as_array().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["name"], "Drawer 1");
}

#[tokio::test]
async fn update_location_name_returns_200() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "Old Name" })),
    )
    .await;
    let lid = created["id"].as_u64().unwrap();

    let (status, body) = common::request(
        &app,
        Method::PATCH,
        &format!("/api/v1/location/{lid}"),
        Some(json!({ "name": "New Name" })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "New Name");
}

#[tokio::test]
async fn delete_location_returns_204() {
    let (app, _dir) = common::make_app();
    let (_, created) = common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "Temp" })),
    )
    .await;
    let lid = created["id"].as_u64().unwrap();

    let (status, _) = common::request(
        &app,
        Method::DELETE,
        &format!("/api/v1/location/{lid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_location_blocked_by_spool_returns_409() {
    let (app, _dir) = common::make_app();
    let fid = create_filament(&app).await;
    let (_, loc) = common::request(
        &app,
        Method::POST,
        "/api/v1/location",
        Some(json!({ "name": "In Use" })),
    )
    .await;
    let lid = loc["id"].as_u64().unwrap();

    // Create a spool assigned to this location
    common::request(
        &app,
        Method::POST,
        "/api/v1/spool",
        Some(json!({ "filament_id": fid, "colors": [], "initial_weight": 1000.0, "location_id": lid })),
    ).await;

    let (status, _) = common::request(
        &app,
        Method::DELETE,
        &format!("/api/v1/location/{lid}"),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}
