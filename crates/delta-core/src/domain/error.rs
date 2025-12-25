use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DomainError {
    #[error("user does not exist")]
    UserDoesNotExist,

    #[error("admin does not exist")]
    AdminDoesNotExist,

    #[error("not authorized to perform this operation")]
    NotAuthorized,

    #[error("approval is required for this operation")]
    ApprovalRequired,

    #[error("amount must be greater than zero")]
    InvalidAmount,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("user must be at least 18 years old")]
    Underage,

    #[error("invalid domain state")]
    InvalidDomainState,
}
