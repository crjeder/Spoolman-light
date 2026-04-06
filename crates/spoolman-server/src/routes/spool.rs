use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use spoolman_types::{
    requests::{CreateSpool, UpdateSpool},
    responses::SpoolResponse,
};

use crate::{routes::error::Result, store::{JsonStore, SpoolFilter}};

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(show).patch(update).delete(destroy))
        .route("/{id}/clone", post(clone))
}

#[derive(Deserialize)]
struct ListParams {
    filament_id: Option<u32>,
    location_id: Option<u32>,
    allow_archived: Option<bool>,
    sort: Option<String>,
    order: Option<String>,
    offset: Option<usize>,
    limit: Option<usize>,
}

async fn list(
    State(store): State<JsonStore>,
    Query(params): Query<ListParams>,
) -> Result<(HeaderMap, Json<Vec<SpoolResponse>>)> {
    let (items, total) = store.list_spools(SpoolFilter {
        filament_id: params.filament_id,
        location_id: params.location_id,
        allow_archived: params.allow_archived.unwrap_or(false),
        sort: params.sort.as_deref(),
        order: params.order.as_deref(),
        offset: params.offset.unwrap_or(0),
        limit: params.limit,
    })?;
    let mut headers = HeaderMap::new();
    headers.insert("X-Total-Count", HeaderValue::from(total as u64));
    Ok((headers, Json(items)))
}

async fn show(State(store): State<JsonStore>, Path(id): Path<u32>) -> Result<Json<SpoolResponse>> {
    Ok(Json(store.get_spool(id)?))
}

async fn create(
    State(store): State<JsonStore>,
    Json(body): Json<CreateSpool>,
) -> Result<(StatusCode, Json<SpoolResponse>)> {
    let created = store.create_spool(body)?;
    Ok((StatusCode::CREATED, Json(created)))
}

async fn update(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateSpool>,
) -> Result<Json<SpoolResponse>> {
    Ok(Json(store.update_spool(id, body)?))
}

async fn destroy(State(store): State<JsonStore>, Path(id): Path<u32>) -> Result<StatusCode> {
    store.delete_spool(id)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn clone(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
) -> Result<(StatusCode, Json<SpoolResponse>)> {
    let cloned = store.clone_spool(id)?;
    Ok((StatusCode::CREATED, Json(cloned)))
}
