use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;

use crate::api::{error::ApiError, response::ApiResult};
use crate::state::AppState;

mod v1;

pub fn routes() -> Router<AppState> {
    Router::new()
        // global (non-versioned) health
        .route("/health", get(health))
        // versioned API
        .nest("/v1", v1::routes())
        .layer(TraceLayer::new_for_http())
        .fallback(|| async { ApiError::NotFound("404") })
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
    version: &'static str,
}

async fn health() -> ApiResult<HealthResponse> {
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            ok: true,
            version: env!("CARGO_PKG_VERSION"),
        }),
    ))
}
