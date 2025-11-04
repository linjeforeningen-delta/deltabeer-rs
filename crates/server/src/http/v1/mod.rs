use axum::{Router, http::StatusCode, response::IntoResponse};

mod admins;
mod stats;
mod users;

pub fn routes() -> Router {
    Router::new()
        .merge(users::routes()) // /v1/users...
        .merge(admins::routes()) // /v1/admins...
        .merge(stats::routes()) // /v1/stats...
        .fallback(|| async { (StatusCode::NOT_FOUND, "v1 404").into_response() })
}
