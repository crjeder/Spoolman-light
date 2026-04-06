use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use spoolman_types::{
    requests::{CreateLocation, UpdateLocation},
    responses::LocationResponse,
};

use crate::{routes::error::Result, store::JsonStore};

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(show).patch(update).delete(destroy))
}

async fn list(State(store): State<JsonStore>) -> Result<Json<Vec<LocationResponse>>> {
    Ok(Json(store.list_locations()?))
}

async fn show(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
) -> Result<Json<LocationResponse>> {
    Ok(Json(store.get_location(id)?))
}

async fn create(
    State(store): State<JsonStore>,
    Json(body): Json<CreateLocation>,
) -> Result<(StatusCode, Json<LocationResponse>)> {
    let created = store.create_location(body)?;
    Ok((StatusCode::CREATED, Json(created)))
}

async fn update(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateLocation>,
) -> Result<Json<LocationResponse>> {
    Ok(Json(store.update_location(id, body)?))
}

async fn destroy(State(store): State<JsonStore>, Path(id): Path<u32>) -> Result<StatusCode> {
    store.delete_location(id)?;
    Ok(StatusCode::NO_CONTENT)
}
