use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod http;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_state = AppState {};
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()) // pretty console output
        .with(tracing_subscriber::EnvFilter::new(
            // log everything from tower_http and axum at info+
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tower_http=info,axum=info,server=debug".into()),
        ))
        .init();

    let app = Router::new()
        .merge(http::routes())
        .with_state(app_state) // attach state on the parent
        .layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = TcpListener::bind("000.0.0.0:0").await?;
    let addr = listener.local_addr()?;
    println!("Listening on http://{}", addr);
    println!("Docs available at http://{}/docs", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
