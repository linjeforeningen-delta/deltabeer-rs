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

#[derive(Debug)]
pub enum RepoError {
    NotFound,
    Conflict,
    StorageFailure,
}
