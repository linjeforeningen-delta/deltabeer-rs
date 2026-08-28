#[derive(Debug)]
pub(crate) enum AppError {
    /// The API returned an error that has no distinct TUI behavior.
    Api,
    Unauthorized,
    Forbidden,
    NotFound,
    InvalidUserIdentifier,
    Conflict,
    BadRequest,
    Transport,
    InvalidResponse,
    Validation(ValidationMessage),
    MissingAuthorization {
        operation: AuthorizationOperation,
    },
    SessionExpired,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum AuthorizationOperation {
    TopUp,
    EndAdminSession,
    CreateUser,
    UpdateUser,
    GrantAdmin,
    RevokeAdmin,
}

#[derive(Debug)]
pub(crate) enum ValidationMessage {
    InvalidAmount,
    CardRequiredTopUp,
    NameRequired,
    UsernameRequired,
    ProgramRequired,
    BirthdateFormat,
    ScanCardFirst,
    InvalidCard,
    UserNotIdentified,
    CardRequiredGrant,
    PasswordRequired,
    ConfirmRequired,
    PasswordsMismatch,
    CardRequiredRevoke,
    AdminRequiredAuth,
}
