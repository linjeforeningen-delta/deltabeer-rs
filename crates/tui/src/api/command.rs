use crate::api::auth::AdminTokenDto;
use crate::api::request::ApiRequest;

pub(crate) struct ApiCommand {
    pub request: ApiRequest,
    pub authorization: Option<AdminTokenDto>,
}
