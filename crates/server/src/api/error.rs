use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use delta_api::{ApiErrorCode, ApiErrorResponse};
use delta_core::services::ServiceError;
#[derive(Debug)]
pub(crate) enum ApiError {
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
    fn code(&self) -> ApiErrorCode {
        match self {
            ApiError::InvalidUserIdentifier => ApiErrorCode::InvalidUserIdentifier,
            ApiError::BadRequest(_) => ApiErrorCode::BadRequest,
            ApiError::NotFound(_) => ApiErrorCode::NotFound,
            ApiError::Conflict(_) => ApiErrorCode::Conflict,
            ApiError::Unauthorized(_) => ApiErrorCode::Unauthorized,
            ApiError::Forbidden(_) => ApiErrorCode::Forbidden,
            ApiError::Internal(_) => ApiErrorCode::InternalError,
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
        let status = self.status();
        let code = self.code();
        match &self {
            ApiError::Internal(_) => tracing::error!(%status, ?code, "request failed internally"),
            ApiError::Unauthorized(_) | ApiError::Forbidden(_) => {
                tracing::debug!(%status, ?code, "request rejected")
            }
            _ => tracing::warn!(%status, ?code, "request rejected"),
        }
        let body = ApiErrorResponse {
            code,
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

        (status, Json(body)).into_response()
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
