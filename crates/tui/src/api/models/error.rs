use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub code: String,
    pub message: Option<String>,
}
