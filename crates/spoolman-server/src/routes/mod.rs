pub mod error;
pub mod filament;
pub mod health;
pub mod location;
pub mod other;
pub mod spool;

use crate::{config::Config, store::JsonStore};
use axum::{
    http::{HeaderValue, Method},
    response::Html,
    routing::get,
    Router,
};

/// The SPA shell — embedded at compile time so the server works even when
/// the cargo-leptos CSR build doesn't generate its own index.html.
const INDEX_HTML: &str = include_str!("../index.html");
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    trace::TraceLayer,
};

async fn serve_index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Build the full Axum router with all middleware applied.
pub fn build_router(store: JsonStore, cfg: &Config) -> Router {
    let site_dir = &cfg.site_root;

    // Only add the embedded index.html fallback when the cargo-leptos site
    // directory doesn't exist.  When it does exist, ServeDir below serves
    // the generated index.html (which contains the correct WASM init call).
    let mut api = Router::new()
        .nest("/api/v1/filament", filament::router())
        .nest("/api/v1/spool", spool::router())
        .nest("/api/v1/location", location::router())
        .nest("/api/v1", other::router())
        .merge(health::router())
        .with_state(store);

    if !site_dir.exists() {
        api = api.route("/", get(serve_index));
    }

    let mut app = api
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

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

    // Serve compiled WASM frontend assets.  The directory is read from
    // LEPTOS_SITE_ROOT (set in the container) or defaults to `target/site`
    // for local dev.  Falls back gracefully when the directory doesn't exist.
    if site_dir.exists() {
        let index = site_dir.join("index.html");
        app = app.fallback_service(
            tower_http::services::ServeDir::new(site_dir)
                .append_index_html_on_directories(true)
                .fallback(tower_http::services::ServeFile::new(index)),
        );
    }

    app
}
