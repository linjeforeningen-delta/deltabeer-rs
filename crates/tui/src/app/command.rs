use crate::api::models::auth::AdminToken;
use crate::api::models::user::UserId;

pub(crate) enum Command {
    LookupUser(String),
    Spend { user_id: UserId, amount: u32 },
    TopUp { user_id: UserId, amount: u32, token: AdminToken },
    RequestAdminAuth { identifier: String, password: String },
}