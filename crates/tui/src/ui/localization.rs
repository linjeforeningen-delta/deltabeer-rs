use crate::app::{
    AppError, AuthorizationOperation, ProgressMessage, StatusMessage, ValidationMessage,
};

pub(crate) fn localize_status(status: &StatusMessage) -> String {
    match status {
        StatusMessage::Ready => t!("status.ready").to_string(),
        StatusMessage::UserLoaded(name) => t!("status.user_loaded", name = name).to_string(),
        StatusMessage::SpendSuccess(amount) => {
            t!("status.spend_success", amount = amount).to_string()
        }
        StatusMessage::TopUpSuccess(amount) => {
            t!("status.topup_success", amount = amount).to_string()
        }
        StatusMessage::SessionStarted => t!("status.session_started").to_string(),
        StatusMessage::SessionEnded => t!("status.session_ended").to_string(),
        StatusMessage::UserCreated(name) => t!("status.user_created", name = name).to_string(),
        StatusMessage::UserUpdated(name) => t!("status.user_updated", name = name).to_string(),
        StatusMessage::AdminGranted(id) => t!("status.admin_granted", id = id).to_string(),
        StatusMessage::AdminRevoked(id) => t!("status.admin_revoked", id = id).to_string(),
        StatusMessage::Progress(progress) => localize_progress(progress),
        StatusMessage::Error(error) => localize_error(error),
        StatusMessage::NoChanges => t!("status.no_changes").to_string(),
    }
}

pub(crate) fn localize_error(error: &AppError) -> String {
    match error {
        AppError::Api => t!("errors.api").to_string(),
        AppError::Unauthorized => t!("errors.unauthorized").to_string(),
        AppError::Forbidden => t!("errors.forbidden").to_string(),
        AppError::NotFound => t!("errors.not_found").to_string(),
        AppError::InvalidUserIdentifier => t!("errors.invalid_user_identifier").to_string(),
        AppError::Conflict => t!("errors.conflict").to_string(),
        AppError::BadRequest => t!("errors.bad_request").to_string(),
        AppError::Transport => t!("errors.network").to_string(),
        AppError::InvalidResponse => t!("errors.invalid_response").to_string(),
        AppError::Validation(validation) => {
            format!(
                "{}: {}",
                t!("errors.validation"),
                localize_validation(validation)
            )
        }
        AppError::MissingAuthorization { operation } => format!(
            "{}: {}",
            t!("errors.authentication"),
            localize_authorization_operation(operation)
        ),
        AppError::SessionExpired => t!("status.session_expired").to_string(),
    }
}

fn localize_validation(validation: &ValidationMessage) -> String {
    let key = match validation {
        ValidationMessage::InvalidAmount => "validation.invalid_amount",
        ValidationMessage::CardRequiredTopUp => "validation.card_required_topup",
        ValidationMessage::NameRequired => "validation.name_required",
        ValidationMessage::UsernameRequired => "validation.username_required",
        ValidationMessage::ProgramRequired => "validation.program_required",
        ValidationMessage::BirthdateFormat => "validation.birthdate_format",
        ValidationMessage::ScanCardFirst => "validation.scan_card_first",
        ValidationMessage::InvalidCard => "validation.invalid_card",
        ValidationMessage::UserNotIdentified => "validation.user_not_identified",
        ValidationMessage::CardRequiredGrant => "validation.card_required_grant",
        ValidationMessage::PasswordRequired => "validation.password_required",
        ValidationMessage::ConfirmRequired => "validation.confirm_required",
        ValidationMessage::PasswordsMismatch => "validation.passwords_mismatch",
        ValidationMessage::CardRequiredRevoke => "validation.card_required_revoke",
        ValidationMessage::AdminRequiredAuth => "validation.admin_required_auth",
    };
    t!(key).to_string()
}

fn localize_authorization_operation(operation: &AuthorizationOperation) -> String {
    let key = match operation {
        AuthorizationOperation::TopUp => "auth_errors.topup",
        AuthorizationOperation::EndAdminSession => "auth_errors.end_session",
        AuthorizationOperation::CreateUser => "auth_errors.create_user",
        AuthorizationOperation::UpdateUser => "auth_errors.update_user",
        AuthorizationOperation::GrantAdmin => "auth_errors.grant",
        AuthorizationOperation::RevokeAdmin => "auth_errors.revoke",
    };
    t!(key).to_string()
}

fn localize_progress(progress: &ProgressMessage) -> String {
    let key = match progress {
        ProgressMessage::LookingUp => "progress.looking_up",
        ProgressMessage::Spending => "progress.spending",
        ProgressMessage::ToppingUp => "progress.topping_up",
        ProgressMessage::Authenticating => "progress.authenticating",
        ProgressMessage::StartingSession => "progress.starting_session",
        ProgressMessage::EndingSession => "progress.ending_session",
        ProgressMessage::CreatingUser => "progress.creating_user",
        ProgressMessage::UpdatingUser => "progress.updating_user",
        ProgressMessage::GrantingAdmin => "progress.granting_admin",
        ProgressMessage::RevokingAdmin => "progress.revoking_admin",
    };
    t!(key).to_string()
}
