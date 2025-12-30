mod admin;
mod clock;
mod token;
mod transaction;
mod user;

pub use admin::*;
pub use clock::*;
pub use token::*;
pub use transaction::*;
pub use user::*;

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
