use std::sync::Arc;

use sea_orm::Database;
use tracing_subscriber::EnvFilter;

use crate::{config::AppConfig, migration::migrate_db, state::AppState};

mod auth;
mod config;
mod entity;
mod error;
mod handlers;
mod middleware;
mod router;
mod state;
mod migration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug")),
        )
        .init();

    let cfg = AppConfig::load();
    tracing::info!(db = %cfg.database_url, port = cfg.port, "starting meydoon");

    let db = Database::connect(&cfg.database_url).await?;
    let db = migrate_db(db).await?;

    let state = AppState {
        db,
        config: Arc::new(cfg.clone()),
    };
    let app = router::build(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", cfg.port)).await?;
    tracing::info!("Meydoon API listening on :{}", cfg.port);
    axum::serve(listener, app).await?;
    Ok(())
}
