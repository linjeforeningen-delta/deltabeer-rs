use crate::api::models::user::UserId;

pub(crate) enum Command {
    LookupUser(String),
    Spend { user_id: UserId, amount: u32 },
    RequestAdminAuth { identifier: String, password: String },
}