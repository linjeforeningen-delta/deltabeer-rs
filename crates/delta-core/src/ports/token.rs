use crate::ports::RepoError;
use crate::services::auth::{AdminToken, TokenData};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait TokenRepo {
    async fn insert_token(
        &self,
        token: AdminToken,
        data: TokenData,
        created_at: DateTime<Utc>,
    ) -> Result<(), RepoError>;
    async fn get_token(&self, token: &AdminToken) -> Result<TokenData, RepoError>;
}
