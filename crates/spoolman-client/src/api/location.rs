use super::{delete, get, patch_json, post_json, ApiError};
use spoolman_types::{
    requests::{CreateLocation, UpdateLocation},
    responses::LocationResponse,
};

pub async fn list_locations() -> Result<Vec<LocationResponse>, ApiError> {
    get("/api/v1/location").await
}

pub async fn create_location(body: &CreateLocation) -> Result<LocationResponse, ApiError> {
    post_json("/api/v1/location", body).await
}

pub async fn update_location(id: u32, body: &UpdateLocation) -> Result<LocationResponse, ApiError> {
    patch_json(&format!("/api/v1/location/{id}"), body).await
}

pub async fn delete_location(id: u32) -> Result<(), ApiError> {
    delete(&format!("/api/v1/location/{id}")).await
}
