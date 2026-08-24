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
    let args = config::Args::parse();
    let config = config::Config::load(&args.config)?;
    let pool = create_pool(
        &config.server.database_url,
        config.server.database_pool_size,
    )?;
    let repo = Arc::new(DieselRepo::new(pool));

    let app_state = AppState {
        repo: repo.clone(),
        token_repo: repo.clone(),
        clock: Arc::new(SystemClock),
        ids: Arc::new(UuidIdGenerator),
        tokens: Arc::new(OpaqueTokenSource),
        auth_policy: config.auth_policy(),
    };
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(config.logging.filter));
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .try_init()
        .ok();

    let app = Router::new()
        .merge(http::routes(app_state.clone()))
        .with_state(app_state) // attach state on the parent
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = TcpListener::bind(&config.server.bind_addr).await?;
    let addr = listener.local_addr()?;
    tracing::info!("server started");
    tracing::info!(%addr, "Server listening on http://{addr}");
    tracing::info!(%addr, "Swagger UI available at http://{addr}/docs");
    axum::serve(listener, app).await?;
    Ok(())
}
