use super::{delete, get, patch_json, post_json, ApiError};
use spoolman_types::{
    requests::{CreateFilament, UpdateFilament},
    responses::FilamentResponse,
};

pub async fn list_filaments(material: Option<&str>) -> Result<Vec<FilamentResponse>, ApiError> {
    let url = match material {
        Some(m) => format!(
            "/api/v1/filament?order=asc&sort=manufacturer&material={}",
            m
        ),
        None => "/api/v1/filament?order=asc&sort=manufacturer".to_string(),
    };
    get(&url).await
}

pub async fn list_materials() -> Result<Vec<String>, ApiError> {
    get("/api/v1/material").await
}

pub async fn get_filament(id: u32) -> Result<FilamentResponse, ApiError> {
    get(&format!("/api/v1/filament/{id}")).await
}

pub async fn create_filament(body: &CreateFilament) -> Result<FilamentResponse, ApiError> {
    post_json("/api/v1/filament", body).await
}

pub async fn update_filament(id: u32, body: &UpdateFilament) -> Result<FilamentResponse, ApiError> {
    patch_json(&format!("/api/v1/filament/{id}"), body).await
}

pub async fn delete_filament(id: u32) -> Result<(), ApiError> {
    delete(&format!("/api/v1/filament/{id}")).await
}
