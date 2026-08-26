use crate::api::{auth::AdminTokenDto, request::ApiRequest};

pub(crate) struct ApiCommand {
    pub request: ApiRequest,
    pub authorization: Option<AdminTokenDto>,
}
