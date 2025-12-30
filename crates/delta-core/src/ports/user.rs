use crate::domain::{ActionRecord, User, UserId};
use crate::ports::RepoError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get(&self, key: &UserId) -> Result<User, RepoError>;
    async fn get_by_name(&self, name: &str) -> Result<User, RepoError>;
    async fn get_by_card(&self, card_number: u32) -> Result<User, RepoError>;
    async fn insert(&self, user: User, record: ActionRecord) -> Result<(), RepoError>;
    async fn update(&self, user: User) -> Result<(), RepoError>; // split update into specific actions?
}
