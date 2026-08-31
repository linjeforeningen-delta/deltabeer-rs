use crate::{
    ports::repo::RepoError,
    services::auth::{AdminToken, TokenData},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Persistence operations for opaque authorization tokens.
///
/// A successful lookup must exclude expired or already-invalidated records so
/// token validation can treat `None` as an invalid token.
#[async_trait]
pub trait TokenRepo: Send + Sync {
    async fn insert_token(
        &self,
        token: AdminToken,
        data: TokenData,
        created_at: DateTime<Utc>,
    ) -> Result<(), RepoError>;
    async fn get_token(
        &self,
        token: &AdminToken,
        dt: DateTime<Utc>,
    ) -> Result<Option<TokenData>, RepoError>;
    async fn expire_token(&self, token: &AdminToken) -> Result<(), RepoError>;
}
