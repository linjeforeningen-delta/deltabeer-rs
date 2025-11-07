use super::error::ApiError;
use axum::Json;
use axum::http::StatusCode;

pub type ApiResponse<T> = (StatusCode, Json<T>);
pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
