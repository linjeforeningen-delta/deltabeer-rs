use super::error::AppError;

#[derive(Debug)]
pub(crate) enum StatusMessage {
    Ready,
    UsersLoaded(usize),
    StatsLoaded,
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
    ListingUsers,
    LoadingStats,
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
