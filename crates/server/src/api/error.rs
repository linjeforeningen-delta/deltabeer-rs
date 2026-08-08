use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use delta_core::services::ServiceError;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: Option<String>,
}
#[derive(Debug)]
pub enum ApiError {
    // Default client errors
    NotFound(&'static str),
    BadRequest(&'static str),
    Conflict(&'static str),
    Unauthorized(&'static str),
    Forbidden(&'static str),

    // Specific errors
    InvalidUserIdentifier,

    // For internal errors (DB failures, etc.)
    Internal(String),
}

impl ApiError {
    fn code(&self) -> &'static str {
        match self {
            ApiError::InvalidUserIdentifier => "invalid_user_identifier",
            ApiError::BadRequest(_) => "bad_request",
            ApiError::NotFound(_) => "not_found",
            ApiError::Conflict(_) => "conflict",
            ApiError::Unauthorized(_) => "unauthorized",
            ApiError::Forbidden(_) => "forbidden",
            ApiError::Internal(_) => "internal_error",
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            ApiError::InvalidUserIdentifier => StatusCode::BAD_REQUEST,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            code: self.code(),
            message: match self {
                ApiError::BadRequest(msg)
                | ApiError::NotFound(msg)
                | ApiError::Conflict(msg)
                | ApiError::Unauthorized(msg)
                | ApiError::Forbidden(msg) => Some(msg.to_string()),

                ApiError::Internal(_) => None, // or Some("internal server error".into())
                ApiError::InvalidUserIdentifier => None,
            },
        };

        (self.status(), Json(body)).into_response()
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError::NotFound("Resource not found"),
            ServiceError::InvalidInput => ApiError::BadRequest("Invalid input provided"),
            ServiceError::Conflict => ApiError::Conflict("Resource conflict"),
            ServiceError::Internal => ApiError::Internal("Internal server error".into()),
            ServiceError::InsufficientBalance => ApiError::Conflict("Insufficient balance"),

            ServiceError::NotAuthorized => ApiError::Unauthorized("Not authorized"),
            ServiceError::ApprovalRequired => ApiError::Unauthorized("Approval required"),
            ServiceError::Underage => ApiError::Forbidden("User is underage"),
            ServiceError::StorageFailure => ApiError::Internal("Storage failure".into()),
        }
    }
}
