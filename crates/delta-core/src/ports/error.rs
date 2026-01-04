use crate::domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("entity not found")]
    NotFound,

    #[error("conflict with existing data")]
    Conflict,

    #[error("storage layer failure")]
    StorageFailure,

    #[error("internal repository error")]
    Internal,
}

impl From<DomainError> for RepoError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::UserDoesNotExist | DomainError::AdminDoesNotExist => RepoError::NotFound,

            DomainError::InsufficientBalance
            | DomainError::NotAuthorized
            | DomainError::ApprovalRequired
            | DomainError::InvalidAmount
            | DomainError::Underage => RepoError::Conflict,

            DomainError::InvalidDomainState | DomainError::InvalidIdent => RepoError::Internal,
        }
    }
}

#[cfg(feature = "diesel")]
impl From<diesel::result::Error> for RepoError {
    fn from(err: diesel::result::Error) -> Self {
        use diesel::result::{DatabaseErrorKind, Error};

        match err {
            Error::NotFound => RepoError::NotFound,

            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)
            | Error::RollbackTransaction => RepoError::Conflict,

            Error::DatabaseError(_, _) => RepoError::StorageFailure,

            _ => RepoError::Internal,
        }
    }
}
