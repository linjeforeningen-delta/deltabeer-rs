use crate::api::models::auth::AdminToken;
use crate::api::request::ApiRequest;

pub(crate) struct ApiCommand {
    pub request: ApiRequest,
    pub authorization: Option<AdminToken>,
}
