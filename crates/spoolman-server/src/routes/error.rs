use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::store::StoreError;

/// Unified error type for route handlers.
pub struct ApiError(StoreError);

impl From<StoreError> for ApiError {
    fn from(e: StoreError) -> Self {
        Self(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self.0 {
            StoreError::NotFound => (StatusCode::NOT_FOUND, self.0.to_string()),
            StoreError::Conflict(_) => (StatusCode::CONFLICT, self.0.to_string()),
            StoreError::Validation(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.0.to_string()),
            StoreError::Io(_) | StoreError::Json(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };
        (status, Json(json!({ "detail": msg }))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
