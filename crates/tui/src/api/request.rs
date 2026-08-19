use crate::api::models::user::{UserId, UserPatch};
use chrono::NaiveDate;

#[derive(Debug)]
pub(crate) enum ApiRequest {
    LookupUser(String),
    Spend {
        user_id: UserId,
        amount: u32,
    },
    TopUp {
        user_id: UserId,
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
    UpdateUser {
        user_id: UserId,
        patch: UserPatch,
    },
    GrantAdmin {
        user_id: UserId,
        password: String,
    },
    RevokeAdmin {
        user_id: UserId,
    },
}

impl ApiRequest {
    pub(crate) fn requires_auth(&self) -> bool {
        matches!(
            self,
            ApiRequest::TopUp { .. }
                | ApiRequest::MakeUser { .. }
                | ApiRequest::UpdateUser { .. }
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
            Self::UpdateUser { .. } => "Updating user...",
            Self::GrantAdmin { .. } => "Granting admin...",
            Self::RevokeAdmin { .. } => "Revoking admin...",
        }
    }
}
