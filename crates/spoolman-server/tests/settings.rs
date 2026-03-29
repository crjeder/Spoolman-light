mod common;

use axum::http::{Method, StatusCode};
use serde_json::json;

#[tokio::test]
async fn put_and_get_setting() {
    let (app, _dir) = common::make_app();

    let (status, _) = common::request(
        &app,
        Method::PUT,
        "/api/v1/setting/theme",
        Some(json!({ "value": "dark" })),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (status, body) = common::request(&app, Method::GET, "/api/v1/setting", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["theme"], "dark");
}
