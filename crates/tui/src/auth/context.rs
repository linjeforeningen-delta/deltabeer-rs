use crate::api::models::user::UserId;

#[derive(Debug, Clone)]
pub(crate) struct AdminContext {
    pub(crate) user_id: UserId,
    pub(crate) name: String,
}
