mod doc;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::response::ApiResult;
use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::get};

/// Mounts:
///   GET /v1/stats
///   GET /v1/stats/summary
pub fn routes() -> Router<AppState> {
    Router::new().nest(
        "/stats",
        Router::new()
            .route("/", get(get_stats)) // GET /v1/stats
            .route("/summary", get(summary)),
    )
}

#[utoipa::path(
    get,
    path = "",
    tag = "stats",
    responses(
        (status = 200, description = "Comprehensive stats", body = StatsDto)
    )
)]
async fn get_stats(State(state): State<AppState>) -> ApiResult<StatsDto> {
    todo!()
}

#[utoipa::path(
    get,
    path = "/summary",
    tag = "stats",
    responses(
        (status = 200, description = "Summary of stats", body = StatsSummaryDto)
    )
)]
async fn summary(State(state): State<AppState>) -> ApiResult<StatsSummaryDto> {
    todo!()
}
