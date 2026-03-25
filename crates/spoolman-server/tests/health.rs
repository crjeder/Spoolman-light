mod common;

use axum::http::{Method, StatusCode};

#[tokio::test]
async fn health_returns_ok() {
    let (app, _dir) = common::make_app();
    let (status, body) = common::request(&app, Method::GET, "/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}
