use crate::{
    domain::{ActionRecord, User, UserId},
    ports::repo::RepoError,
};
use async_trait::async_trait;

/// Persistence operations for users and user lookup identities.
///
/// Repository implementations are responsible for translating storage errors
/// into the normalized `RepoError` values consumed by services.
#[async_trait]
pub trait UserRepo {
    async fn get_user(&self, key: &UserId) -> Result<User, RepoError>;
    async fn get_user_by_name(&self, name: &str) -> Result<User, RepoError>;
    async fn get_user_by_card(&self, card_number: u32) -> Result<User, RepoError>;
    async fn list_users(&self) -> Result<Vec<User>, RepoError>;
    async fn list_admins(&self) -> Result<Vec<User>, RepoError>;
    async fn insert_user(&self, user: User, record: ActionRecord) -> Result<(), RepoError>;
    async fn update_user(&self, user: User) -> Result<(), RepoError>; // split update into specific actions?
}
