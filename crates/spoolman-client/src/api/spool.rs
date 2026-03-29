use super::{delete, get, patch_json, post_json, ApiError};
use spoolman_types::{
    requests::{CreateSpool, UpdateSpool},
    responses::SpoolResponse,
};

pub async fn list_spools(allow_archived: bool) -> Result<Vec<SpoolResponse>, ApiError> {
    get(&format!(
        "/api/v1/spool?allow_archived={allow_archived}&order=desc"
    ))
    .await
}

pub async fn get_spool(id: u32) -> Result<SpoolResponse, ApiError> {
    get(&format!("/api/v1/spool/{id}")).await
}

pub async fn create_spool(body: &CreateSpool) -> Result<SpoolResponse, ApiError> {
    post_json("/api/v1/spool", body).await
}

pub async fn update_spool(id: u32, body: &UpdateSpool) -> Result<SpoolResponse, ApiError> {
    patch_json(&format!("/api/v1/spool/{id}"), body).await
}

pub async fn delete_spool(id: u32) -> Result<(), ApiError> {
    delete(&format!("/api/v1/spool/{id}")).await
}

pub async fn clone_spool(id: u32) -> Result<SpoolResponse, ApiError> {
    post_json(
        &format!("/api/v1/spool/{id}/clone"),
        &serde_json::Value::Null,
    )
    .await
}
