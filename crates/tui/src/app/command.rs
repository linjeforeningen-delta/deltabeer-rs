use crate::api::models::auth::AdminToken;
use crate::api::models::user::UserId;
use chrono::NaiveDate;

pub(crate) enum Command {
    LookupUser(String),
    Spend { user_id: UserId, amount: u32 },
    TopUp { user_id: UserId, amount: u32, token: AdminToken },
    RequestAdminAuth { identifier: String, password: String },
    MakeUser {
        name: String,
        username: String,
        program: String,
        card_number: u32,
        birthdate: NaiveDate,
        token: AdminToken,
    },
}