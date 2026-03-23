pub mod error;
pub mod filament;
pub mod health;
pub mod location;
pub mod other;
pub mod spool;

use crate::{config::Config, store::JsonStore};
use axum::{
    http::{HeaderValue, Method},
    Router,
};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

/// Build the full Axum router with all middleware applied.
pub fn build_router(store: JsonStore, cfg: &Config) -> Router {
    let api = Router::new()
        .nest("/api/v1/filament", filament::router())
        .nest("/api/v1/spool", spool::router())
        .nest("/api/v1/location", location::router())
        .nest("/api/v1", other::router())
        .merge(health::router())
        .with_state(store);

    let mut app = api.layer(CompressionLayer::new()).layer(TraceLayer::new_for_http());

    if let Some(origin) = &cfg.cors_origin {
        let allowed: AllowOrigin = if origin == "*" {
            AllowOrigin::any()
        } else {
            let hv = HeaderValue::from_str(origin).expect("invalid CORS origin");
            AllowOrigin::exact(hv)
        };
        app = app.layer(
            CorsLayer::new()
                .allow_origin(allowed)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(tower_http::cors::Any),
        );
    }

    // Serve compiled WASM frontend assets.  The directory is resolved from the
    // LEPTOS_SITE_ROOT env var (set to /site in the Docker image) with a fallback
    // to "target/site" for local `cargo leptos watch` development.
    let site_dir = std::env::var("LEPTOS_SITE_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("target/site"));
    if site_dir.exists() {
        let index = site_dir.join("index.html");
        app = app.fallback_service(
            tower_http::services::ServeDir::new(&site_dir)
                .fallback(tower_http::services::ServeFile::new(index)),
        );
    }

    app
}
