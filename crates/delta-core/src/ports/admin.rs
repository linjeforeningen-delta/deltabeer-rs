use crate::domain::{ActionRecord, UserId};
use crate::ports::RepoError;
use async_trait::async_trait;

pub struct Admin {
    id: String,
    password_hash: String,
}

#[async_trait]
pub trait AdminRepo {
    async fn get(&self, id: UserId) -> Result<String, RepoError>;
    async fn grant(&self, id: UserId, data: String, record: ActionRecord) -> Result<(), RepoError>;
    async fn revoke(&self, id: UserId, record: ActionRecord) -> Result<(), RepoError>;
}
