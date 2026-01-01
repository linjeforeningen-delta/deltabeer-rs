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
