use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    pub message: String,
}
#[derive(Debug)]
pub enum ApiError {
    NotFound(&'static str),
    BadRequest(&'static str),
    Conflict(&'static str),
    Unauthorized(&'static str),
    Forbidden(&'static str),

    // For internal errors (DB failures, etc.)
    Internal(String),
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.to_string()),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.to_string()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.to_string()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.to_string()),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.to_string()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, message).into_response()
    }
}
