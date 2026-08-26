use crate::{
    domain::UserId,
    ports::{Clock, TokenRepo},
    services::auth::{AdminToken, TokenKind},
};
use async_trait::async_trait;
use chrono::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Failed to issue token")]
    FailedToIssueToken,
}

#[async_trait]
pub trait TokenSource: Send + Sync {
    async fn issue_token(
        &self,
        user_id: UserId,
        ttl: Duration,
        kind: TokenKind,
        repo: &(dyn TokenRepo + Sync),
        clock: &(dyn Clock + Sync),
    ) -> Result<AdminToken, TokenError>;

    async fn expire_token(
        &self,
        token: AdminToken,
        repo: &(dyn TokenRepo + Sync),
    ) -> Result<(), TokenError>;

    async fn validate_token(
        &self,
        token: AdminToken,
        repo: &(dyn TokenRepo + Sync),
        clock: &(dyn Clock + Sync),
    ) -> Result<UserId, TokenError>;
}
