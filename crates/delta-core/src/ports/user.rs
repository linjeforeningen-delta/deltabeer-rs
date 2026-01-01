use crate::domain::{ActionRecord, User, UserId};
use crate::ports::RepoError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_user(&self, key: &UserId) -> Result<User, RepoError>;
    async fn get_user_by_name(&self, name: &str) -> Result<User, RepoError>;
    async fn get_user_by_card(&self, card_number: u32) -> Result<User, RepoError>;
    async fn insert_user(&self, user: User, record: ActionRecord) -> Result<(), RepoError>;
    async fn update_user(&self, user: User) -> Result<(), RepoError>; // split update into specific actions?
}
