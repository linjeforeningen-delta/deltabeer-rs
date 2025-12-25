use crate::domain::User;
use crate::ports::RepoError;
use async_trait::async_trait;
use thiserror::Error;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get(&self, key: &str) -> Result<User, RepoError>;
    async fn get_by_name(&self, name: &str) -> Result<User, RepoError>;
    async fn get_by_card(&self, card_number: u32) -> Result<User, RepoError>;
    async fn insert(&self, user: User) -> Result<(), RepoError>;
    async fn update(&self, user: User) -> Result<(), RepoError>;
}
