use crate::ports::RepoError;
use crate::services::auth::{AdminToken, TokenData};
use async_trait::async_trait;

#[async_trait]
pub trait TokenRepo {
    async fn insert(&self, token: AdminToken, data: TokenData) -> Result<(), RepoError>;
    async fn get(&self, token: &AdminToken) -> Result<TokenData, RepoError>;
    async fn remove(&self, token: &AdminToken) -> Result<(), RepoError>;
}
