use crate::app::{ProgressMessage, StatusMessage};
use crate::model::{UserId, UserPatch};
use chrono::NaiveDate;

#[derive(Debug)]
pub(crate) enum ApiRequest {
    ListUsers,
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

    pub(crate) fn status_message(&self) -> StatusMessage {
        let progress = match self {
            Self::ListUsers => ProgressMessage::ListingUsers,
            Self::LookupUser(_) => ProgressMessage::LookingUp,
            Self::Spend { .. } => ProgressMessage::Spending,
            Self::TopUp { .. } => ProgressMessage::ToppingUp,
            Self::AuthenticateAdmin { .. } => ProgressMessage::Authenticating,
            Self::StartAdminSession { .. } => ProgressMessage::StartingSession,
            Self::EndAdminSession => ProgressMessage::EndingSession,
            Self::MakeUser { .. } => ProgressMessage::CreatingUser,
            Self::UpdateUser { .. } => ProgressMessage::UpdatingUser,
            Self::GrantAdmin { .. } => ProgressMessage::GrantingAdmin,
            Self::RevokeAdmin { .. } => ProgressMessage::RevokingAdmin,
        };
        StatusMessage::Progress(progress)
    }
}
