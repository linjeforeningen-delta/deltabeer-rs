//! DeltaBeer HTTP server binary.
//!
//! This crate composes configuration, Axum HTTP routing, OpenAPI metadata,
//! request mappings, core services, and the SQLite repository.
//!
//! It owns application wiring and transport concerns; business rules remain
//! in `delta-core`, while `delta-api` supplies the shared wire contract.

use axum::Router;
use clap::Parser;
use delta_core::infra::{clock::SystemClock, id::UuidIdGenerator, token::OpaqueTokenSource};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod http;
mod state;

use state::AppState;
use storage_diesel::{DieselRepo, create_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (filter, filter_handle) = tracing_subscriber::reload::Layer::new(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
    );
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .try_init()
        .ok();

    let args = config::Args::parse();
    let config = match config::Config::load(&args.config) {
        Ok(config) => config,
        Err(error) => {
            tracing::error!(error = %error, path = %args.config.display(), "failed to load configuration");
            return Err(error);
        }
    };
    if std::env::var_os("RUST_LOG").is_none()
        && let Err(error) = filter_handle.reload(EnvFilter::new(config.logging.filter.clone()))
    {
        tracing::warn!(error = %error, "failed to apply configured log filter");
    }

    let pool = match create_pool(
        &config.server.database_url,
        config.server.database_pool_size,
    ) {
        Ok(pool) => pool,
        Err(error) => {
            tracing::error!(error = %error, "failed to initialize database pool");
            return Err(anyhow::Error::from(error));
        }
    };
    let repo = Arc::new(DieselRepo::new(pool));

    let app_state = AppState {
        repo: repo.clone(),
        token_repo: repo.clone(),
        clock: Arc::new(SystemClock),
        ids: Arc::new(UuidIdGenerator),
        tokens: Arc::new(OpaqueTokenSource),
        auth_policy: config.auth_policy(),
    };
    let app = Router::new()
        .merge(http::routes(app_state.clone()))
        .with_state(app_state) // attach state on the parent
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = match TcpListener::bind(&config.server.bind_addr).await {
        Ok(listener) => listener,
        Err(error) => {
            tracing::error!(error = %error, bind_addr = %config.server.bind_addr, "failed to bind server listener");
            return Err(anyhow::Error::from(error));
        }
    };
    let addr = match listener.local_addr() {
        Ok(addr) => addr,
        Err(error) => {
            tracing::error!(error = %error, "failed to determine server address");
            return Err(anyhow::Error::from(error));
        }
    };
    tracing::info!("server started");
    tracing::info!(%addr, "Server listening on http://{addr}");
    tracing::info!(%addr, "Swagger UI available at http://{addr}/docs");
    if let Err(error) = axum::serve(listener, app).await {
        tracing::error!(error = %error, "server stopped unexpectedly");
        return Err(anyhow::Error::from(error));
    }
    Ok(())
}
