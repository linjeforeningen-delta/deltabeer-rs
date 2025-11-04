use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;
use tower_http::trace::TraceLayer;

mod v1;

pub fn routes() -> Router {
    Router::new()
        // global (non-versioned) health
        .route("/health", get(health))
        // versioned API
        .nest("/v1", v1::routes())
        .layer(TraceLayer::new_for_http())
        .fallback(|| async { (StatusCode::NOT_FOUND, "404").into_response() })
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({"ok": true, "version": env!("CARGO_PKG_VERSION")}))
}
