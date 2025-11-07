use super::_types::*;
use crate::api::response::ApiResult;
use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::get};

/// Mounts:
///   GET /v1/stats
///   GET /v1/stats/summary
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/stats", get(get_stats))
        .nest("/stats", Router::new().route("/summary", get(summary)))
}

async fn get_stats(State(state): State<AppState>) -> ApiResult<StatsDto> {
    todo!("Implement a stats query")
}

async fn summary(State(state): State<AppState>) -> ApiResult<StatsSummaryDto> {
    todo!("Implement a stats summary")
}
