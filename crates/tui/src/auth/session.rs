use crate::api::models::auth::SessionToken;
use crate::api::models::user::UserId;

#[derive(Debug, Clone)]
pub(crate) struct AdminSession {
    pub(crate) user_id: UserId,
    pub(crate) token: SessionToken,
}

impl AdminSession {
    pub(crate) fn new(user_id: UserId, token: SessionToken) -> Self {
        Self { user_id, token }
    }
}
