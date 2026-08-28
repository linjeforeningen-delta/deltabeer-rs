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

impl AppError {
    pub(crate) fn localized(&self) -> String {
        match self {
            Self::Api => t!("errors.api").to_string(),
            Self::Unauthorized => t!("errors.unauthorized").to_string(),
            Self::Forbidden => t!("errors.forbidden").to_string(),
            Self::NotFound => t!("errors.not_found").to_string(),
            Self::InvalidUserIdentifier => t!("errors.invalid_user_identifier").to_string(),
            Self::Conflict => t!("errors.conflict").to_string(),
            Self::BadRequest => t!("errors.bad_request").to_string(),
            Self::Transport => t!("errors.network").to_string(),
            Self::InvalidResponse => t!("errors.invalid_response").to_string(),
            Self::Validation(validation) => {
                format!("{}: {}", t!("errors.validation"), validation.localized())
            }
            Self::MissingAuthorization { operation } => {
                format!("{}: {}", t!("errors.authentication"), operation.localized())
            }
            Self::SessionExpired => t!("status.session_expired").to_string(),
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

impl ValidationMessage {
    fn localized(&self) -> String {
        let key = match self {
            Self::InvalidAmount => "validation.invalid_amount",
            Self::CardRequiredTopUp => "validation.card_required_topup",
            Self::NameRequired => "validation.name_required",
            Self::UsernameRequired => "validation.username_required",
            Self::ProgramRequired => "validation.program_required",
            Self::BirthdateFormat => "validation.birthdate_format",
            Self::ScanCardFirst => "validation.scan_card_first",
            Self::InvalidCard => "validation.invalid_card",
            Self::UserNotIdentified => "validation.user_not_identified",
            Self::CardRequiredGrant => "validation.card_required_grant",
            Self::PasswordRequired => "validation.password_required",
            Self::ConfirmRequired => "validation.confirm_required",
            Self::PasswordsMismatch => "validation.passwords_mismatch",
            Self::CardRequiredRevoke => "validation.card_required_revoke",
            Self::AdminRequiredAuth => "validation.admin_required_auth",
        };
        t!(key).to_string()
    }
}

impl AuthorizationOperation {
    fn localized(&self) -> String {
        let key = match self {
            Self::TopUp => "auth_errors.topup",
            Self::EndAdminSession => "auth_errors.end_session",
            Self::CreateUser => "auth_errors.create_user",
            Self::UpdateUser => "auth_errors.update_user",
            Self::GrantAdmin => "auth_errors.grant",
            Self::RevokeAdmin => "auth_errors.revoke",
        };
        t!(key).to_string()
    }
}
