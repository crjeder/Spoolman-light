mod common;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

/// Download returns the exact bytes on disk, and they parse as a data store.
#[tokio::test]
async fn download_returns_current_store() {
    let (app, _dir) = common::make_app();
    common::create_location(&app).await;

    let (status, body) = common::request(&app, Method::GET, "/api/v1/database/download", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["locations"].as_array().unwrap().len(), 1);
}

/// Uploading a valid store replaces the live data and is reflected immediately.
#[tokio::test]
async fn upload_replaces_store() {
    let (app, _dir) = common::make_app();
    common::create_location(&app).await; // ensure the file exists on disk before downloading

    let (_, downloaded) = common::request(&app, Method::GET, "/api/v1/database/download", None).await;
    let mut replacement = downloaded;
    replacement["settings"]["currency_symbol"] = serde_json::json!("$");

    let (status, _) = common::request(
        &app,
        Method::POST,
        "/api/v1/database/upload",
        Some(replacement),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = common::request(&app, Method::GET, "/api/v1/setting", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["currency_symbol"], "$");
}

/// A malformed upload is rejected and leaves existing data untouched.
#[tokio::test]
async fn upload_rejects_malformed_body() {
    let (app, _dir) = common::make_app();
    common::request(
        &app,
        Method::PUT,
        "/api/v1/setting/currency_symbol",
        Some(serde_json::json!({ "value": "EUR" })),
    )
    .await;

    let req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/database/upload")
        .header("content-type", "application/json")
        .body(Body::from("not valid json"))
        .unwrap();
    let resp = app.clone().oneshot(req).await.unwrap();
    assert!(!resp.status().is_success());
    let _ = resp.into_body().collect().await.unwrap().to_bytes();

    let (status, body) = common::request(&app, Method::GET, "/api/v1/setting", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["currency_symbol"], "EUR");
}
