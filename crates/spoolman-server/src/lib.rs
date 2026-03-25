pub mod config;
pub mod routes;
pub mod store;
mod backup;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run() {
    dotenvy::dotenv().ok();
    let cfg = config::Config::from_env();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| cfg.logging_level.as_str().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(version = %cfg.version, "starting spoolman-server");

    let store = store::JsonStore::load(&cfg.data_file).expect("failed to load data store");
    tracing::info!(path = %cfg.data_file.display(), "data store loaded");

    if cfg.automatic_backup {
        backup::start(cfg.data_file.clone());
    }

    let app = routes::build_router(store, &cfg);

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port)
        .parse()
        .expect("invalid bind address");

    tracing::info!(%addr, "listening");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
