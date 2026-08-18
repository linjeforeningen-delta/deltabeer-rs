use crate::api::models::user::UserId;
use chrono::NaiveDate;

#[derive(Debug)]
pub(crate) enum ApiRequest {
    LookupUser(String),
    Spend {
        user_id: UserId,
        amount: u32,
    },
    TopUp {
        identifier: String,
        amount: u32,
    },
    AuthenticateAdmin {
        user_id: UserId,
        password: String,
    },
    MakeUser {
        name: String,
        username: String,
        program: String,
        card_number: u32,
        birthdate: NaiveDate,
    },
    GrantAdmin {
        identifier: String,
        password: String,
    },
    RevokeAdmin {
        identifier: String,
    },
}

impl ApiRequest {
    pub(crate) fn requires_auth(&self) -> bool {
        matches!(
            self,
            ApiRequest::TopUp { .. }
                | ApiRequest::GrantAdmin { .. }
                | ApiRequest::RevokeAdmin { .. }
        )
    }

    pub(crate) fn status_message(&self) -> &'static str {
        match self {
            Self::LookupUser(_) => "Looking up user...",
            Self::Spend { .. } => "Spending...",
            Self::TopUp { .. } => "Topping up...",
            Self::AuthenticateAdmin { .. } => "Authenticating admin...",
            Self::MakeUser { .. } => "Creating user...",
            Self::GrantAdmin { .. } => "Granting admin...",
            Self::RevokeAdmin { .. } => "Revoking admin...",
        }
    }
}