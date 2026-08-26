use crate::api::error::ApiError;
use crate::state::AppState;
use axum::Router;
use utoipa::OpenApi;

pub(super) mod admins;
pub(super) mod stats;
pub(super) mod users;

pub(super) fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(users::routes()) // /v1/users...
        .merge(admins::routes(state.clone())) // /v1/admins...
        .merge(stats::routes()) // /v1/stats...
        .fallback(|| async { ApiError::NotFound("404") })
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/users", api = users::ApiDoc),
        (path = "/admins", api = admins::ApiDoc),
        (path = "/stats", api = stats::ApiDoc),
    )
)]
pub(super) struct ApiDoc;
