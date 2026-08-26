use super::error::ApiError;
use axum::{Json, http::StatusCode};

pub type ApiResponse<T> = (StatusCode, Json<T>);
pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
