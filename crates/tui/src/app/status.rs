use super::error::AppError;

#[derive(Debug)]
pub(crate) enum StatusMessage {
    Ready,
    UserLoaded(String),
    SpendSuccess(u32),
    TopUpSuccess(u32),
    SessionStarted,
    SessionEnded,
    UserCreated(String),
    UserUpdated(String),
    AdminGranted(String),
    AdminRevoked(String),
    Progress(ProgressMessage),
    Error(AppError),
    NoChanges,
}

#[derive(Debug)]
pub(crate) enum ProgressMessage {
    LookingUp,
    Spending,
    ToppingUp,
    Authenticating,
    StartingSession,
    EndingSession,
    CreatingUser,
    UpdatingUser,
    GrantingAdmin,
    RevokingAdmin,
}

impl StatusMessage {
    pub(crate) fn localized(&self) -> String {
        match self {
            Self::Ready => t!("status.ready").to_string(),
            Self::UserLoaded(name) => t!("status.user_loaded", name = name).to_string(),
            Self::SpendSuccess(amount) => t!("status.spend_success", amount = amount).to_string(),
            Self::TopUpSuccess(amount) => t!("status.topup_success", amount = amount).to_string(),
            Self::SessionStarted => t!("status.session_started").to_string(),
            Self::SessionEnded => t!("status.session_ended").to_string(),
            Self::UserCreated(name) => t!("status.user_created", name = name).to_string(),
            Self::UserUpdated(name) => t!("status.user_updated", name = name).to_string(),
            Self::AdminGranted(id) => t!("status.admin_granted", id = id).to_string(),
            Self::AdminRevoked(id) => t!("status.admin_revoked", id = id).to_string(),
            Self::Progress(progress) => progress.localized(),
            Self::Error(error) => error.localized(),
            Self::NoChanges => t!("status.no_changes").to_string(),
        }
    }
}

impl ProgressMessage {
    fn localized(&self) -> String {
        let key = match self {
            Self::LookingUp => "progress.looking_up",
            Self::Spending => "progress.spending",
            Self::ToppingUp => "progress.topping_up",
            Self::Authenticating => "progress.authenticating",
            Self::StartingSession => "progress.starting_session",
            Self::EndingSession => "progress.ending_session",
            Self::CreatingUser => "progress.creating_user",
            Self::UpdatingUser => "progress.updating_user",
            Self::GrantingAdmin => "progress.granting_admin",
            Self::RevokingAdmin => "progress.revoking_admin",
        };
        t!(key).to_string()
    }
}
