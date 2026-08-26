use super::error::ApiError;
use axum::{Json, http::StatusCode};

pub(crate) type ApiResponse<T> = (StatusCode, Json<T>);
pub(crate) type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
