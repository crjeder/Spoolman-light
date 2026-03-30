use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use spoolman_types::{
    requests::{CreateFilament, UpdateFilament},
    responses::{FilamentResponse, SpoolmanDbEntry},
};

use crate::{routes::error::Result, store::JsonStore};

pub fn router() -> Router<JsonStore> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/search", get(search))
        .route("/:id", get(show).patch(update).delete(destroy))
}

#[derive(Deserialize)]
struct ListParams {
    material: Option<String>,
    sort: Option<String>,
    order: Option<String>,
    offset: Option<usize>,
    limit: Option<usize>,
}

async fn list(
    State(store): State<JsonStore>,
    Query(params): Query<ListParams>,
) -> Result<(HeaderMap, Json<Vec<FilamentResponse>>)> {
    let (items, total) = store.list_filaments(
        params.material.as_deref(),
        params.sort.as_deref(),
        params.order.as_deref(),
        params.offset.unwrap_or(0),
        params.limit,
    )?;
    let mut headers = HeaderMap::new();
    headers.insert("X-Total-Count", HeaderValue::from(total as u64));
    Ok((headers, Json(items)))
}

async fn show(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
) -> Result<Json<FilamentResponse>> {
    Ok(Json(store.get_filament(id)?))
}

async fn create(
    State(store): State<JsonStore>,
    Json(body): Json<CreateFilament>,
) -> Result<(axum::http::StatusCode, Json<FilamentResponse>)> {
    let created = store.create_filament(body)?;
    Ok((axum::http::StatusCode::CREATED, Json(created)))
}

async fn update(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
    Json(body): Json<UpdateFilament>,
) -> Result<Json<FilamentResponse>> {
    Ok(Json(store.update_filament(id, body)?))
}

async fn destroy(
    State(store): State<JsonStore>,
    Path(id): Path<u32>,
) -> Result<axum::http::StatusCode> {
    store.delete_filament(id)?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

// ── SpoolmanDB search proxy ────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

async fn search(Query(params): Query<SearchParams>) -> Result<Json<Vec<SpoolmanDbEntry>>> {
    let q = params.q.unwrap_or_default();
    let url = "https://donkie.github.io/SpoolmanDB/filaments.json".to_string();
    let entries = fetch_spoolmandb(&url, &q).await.unwrap_or_default();
    Ok(Json(entries))
}

async fn fetch_spoolmandb(url: &str, query: &str) -> Option<Vec<SpoolmanDbEntry>> {
    let resp = reqwest::get(url).await.ok()?;
    let raw: Vec<serde_json::Value> = resp.json().await.ok()?;
    let q = query.to_lowercase();
    let results = raw
        .into_iter()
        .filter(|entry| {
            if q.is_empty() {
                return true;
            }
            let text = format!(
                "{} {} {}",
                entry
                    .get("manufacturer")
                    .and_then(|v| v.as_str())
                    .unwrap_or(""),
                entry.get("material").and_then(|v| v.as_str()).unwrap_or(""),
                entry.get("name").and_then(|v| v.as_str()).unwrap_or(""),
            )
            .to_lowercase();
            text.contains(&q)
        })
        .filter_map(|entry| serde_json::from_value(entry).ok())
        .collect();
    Some(results)
}
