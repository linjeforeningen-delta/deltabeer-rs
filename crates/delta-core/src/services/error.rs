use crate::{
    domain::DomainError,
    ports::{TokenError, repo::RepoError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("invalid input")]
    InvalidInput,

    #[error("not found")]
    NotFound,

    #[error("not authorized")]
    NotAuthorized,

    #[error("conflict")]
    Conflict,

    #[error("approval required")]
    ApprovalRequired,

    #[error("underage")]
    Underage,

    #[error("insufficient balance")]
    InsufficientBalance,

    #[error("storage failure")]
    StorageFailure,

    #[error("internal error")]
    Internal,
}

impl From<DomainError> for ServiceError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::UserDoesNotExist | DomainError::AdminDoesNotExist => {
                ServiceError::NotFound
            }

            DomainError::NotAuthorized => ServiceError::NotAuthorized,

            DomainError::ApprovalRequired => ServiceError::ApprovalRequired,

            DomainError::Underage => ServiceError::Underage,

            DomainError::InsufficientBalance => ServiceError::InsufficientBalance,

            DomainError::InvalidAmount | DomainError::InvalidIdent => ServiceError::InvalidInput,

            _ => ServiceError::Internal,
        }
    }
}

impl From<RepoError> for ServiceError {
    fn from(err: RepoError) -> Self {
        match err {
            RepoError::NotFound => ServiceError::NotFound,

            RepoError::Conflict => ServiceError::Conflict,

            RepoError::StorageFailure => ServiceError::StorageFailure,

            RepoError::Internal => ServiceError::Internal,
        }
    }
}

impl From<TokenError> for ServiceError {
    fn from(err: TokenError) -> Self {
        match err {
            TokenError::InvalidToken => ServiceError::NotAuthorized,
            TokenError::FailedToIssueToken => ServiceError::Internal,
        }
    }
}
