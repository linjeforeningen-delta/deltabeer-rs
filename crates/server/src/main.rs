use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod http;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState {};
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
