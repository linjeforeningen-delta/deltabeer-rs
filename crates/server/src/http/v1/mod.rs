use crate::api::error::ApiError;
use crate::state::AppState;
use axum::{Router, http::StatusCode, response::IntoResponse};
use utoipa::OpenApi;

pub mod admins;
mod mappings;
pub mod stats;
mod types;
pub mod users;

use admins::ApiDoc as AdminsApiDoc;
use stats::ApiDoc as StatsApiDoc;
use users::ApiDoc as UsersApiDoc;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(users::routes()) // /v1/users...
        .merge(admins::routes()) // /v1/admins...
        .merge(stats::routes()) // /v1/stats...
        .fallback(|| async { ApiError::NotFound("404") })
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/users", api = UsersApiDoc),
        (path = "/admins", api = AdminsApiDoc),
        (path = "/stats", api = StatsApiDoc),
    )
)]
pub struct ApiDoc;
