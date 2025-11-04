use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod http; // your handlers live here

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()) // pretty console output
        .with(tracing_subscriber::EnvFilter::new(
            // log everything from tower_http and axum at info+
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tower_http=info,axum=info,server=debug".into()),
        ))
        .init();

    let app = http::routes().layer(tower_http::trace::TraceLayer::new_for_http());

    let listener = TcpListener::bind("127.0.0.1:0").await?;
    println!("Listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}
