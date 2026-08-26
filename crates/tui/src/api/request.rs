use crate::model::{UserId, UserPatch};
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
    StartAdminSession {
        user_id: UserId,
        password: String,
    },
    EndAdminSession,
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

    pub(crate) fn status_message(&self) -> String {
        match self {
            Self::LookupUser(_) => t!("progress.looking_up"),
            Self::Spend { .. } => t!("progress.spending"),
            Self::TopUp { .. } => t!("progress.topping_up"),
            Self::AuthenticateAdmin { .. } => t!("progress.authenticating"),
            Self::StartAdminSession { .. } => t!("progress.starting_session"),
            Self::EndAdminSession => t!("progress.ending_session"),
            Self::MakeUser { .. } => t!("progress.creating_user"),
            Self::UpdateUser { .. } => t!("progress.updating_user"),
            Self::GrantAdmin { .. } => t!("progress.granting_admin"),
            Self::RevokeAdmin { .. } => t!("progress.revoking_admin"),
        }
        .to_string()
    }
}
