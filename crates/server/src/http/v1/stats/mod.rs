mod doc;
pub(super) use doc::ApiDoc;

use crate::api::{mappings, response::ApiResult};
use crate::state::AppState;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use delta_api::{StatsDto, StatsSummaryDto};

pub(super) fn routes() -> Router<AppState> {
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
    let stats = delta_core::services::stats::get_stats(&*state.repo).await?;
    Ok((StatusCode::OK, Json(mappings::stats_to_dto(&stats))))
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
    let stats = delta_core::services::stats::get_stats(&*state.repo).await?;
    Ok((StatusCode::OK, Json(mappings::stats_summary_to_dto(&stats))))
}
