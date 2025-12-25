use crate::domain::{User, UserId};
use crate::ports::RepoError;
use async_trait::async_trait;

pub struct Admin {
    id: String,
    password_hash: String,
}

#[async_trait]
pub trait AdminRepo {
    async fn get(&self, id: UserId) -> Result<String, RepoError>;
    async fn insert(&self, id: UserId, data: String) -> Result<(), RepoError>;
    async fn remove(&self, id: UserId) -> Result<(), RepoError>;
}
