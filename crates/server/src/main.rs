use axum::Router;
use delta_core::infra::{clock::SystemClock, id::UuidIdGenerator, token::OpaqueTokenSource};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod api;
mod http;
mod state;

use state::AppState;
use storage_diesel::{create_pool, DieselRepo, DEV_SQLITE_PATH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DEV_SQLITE_PATH;
    let pool = create_pool(db_url)?;
    let repo = DieselRepo::new(pool);

    let app_state = AppState {
        repo: Arc::new(repo),
        clock: Arc::new(SystemClock),
        ids: Arc::new(UuidIdGenerator),
        tokens: Arc::new(OpaqueTokenSource),
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("tower_http=info,axum=info,server=debug"));
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .try_init()
        .ok();

    let app = Router::new()
        .merge(http::routes())
        .with_state(app_state) // attach state on the parent
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = TcpListener::bind("000.0.0.0:0").await?;
    let addr = listener.local_addr()?;
    tracing::info!("server started");
    tracing::info!(%addr, "Server listening on http://{addr}");
    tracing::info!(%addr, "Swagger UI available at http://{addr}/docs");
    axum::serve(listener, app).await?;
    Ok(())
}
