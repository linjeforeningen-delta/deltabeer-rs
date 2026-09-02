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
    #[expect(
        dead_code,
        reason = "part of the application error model for session-expiration handling"
    )]
    SessionExpired,
}

impl AppError {
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::Api => "api",
            Self::Unauthorized => "unauthorized",
            Self::Forbidden => "forbidden",
            Self::NotFound => "not_found",
            Self::InvalidUserIdentifier => "invalid_user_identifier",
            Self::Conflict => "conflict",
            Self::BadRequest => "bad_request",
            Self::Transport => "transport",
            Self::InvalidResponse => "invalid_response",
            Self::Validation(_) => "validation",
            Self::MissingAuthorization { .. } => "missing_authorization",
            Self::SessionExpired => "session_expired",
        }
    }
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
