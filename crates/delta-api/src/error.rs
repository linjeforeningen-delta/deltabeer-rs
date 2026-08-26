use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    InvalidUserIdentifier,
    BadRequest,
    NotFound,
    Conflict,
    Unauthorized,
    Forbidden,
    InternalError,
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidUserIdentifier => "invalid_user_identifier",
            Self::BadRequest => "bad_request",
            Self::NotFound => "not_found",
            Self::Conflict => "conflict",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::InternalError => "internal_error",
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiErrorResponse {
    pub code: ApiErrorCode,
    pub message: Option<String>,
}
