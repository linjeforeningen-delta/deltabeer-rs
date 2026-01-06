use crate::domain::UserId;
use crate::ports::{Clock, TokenRepo};
use crate::services::auth::{AdminToken, TokenKind};
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

#[async_trait(?Send)]
pub trait TokenSource {
    async fn issue_token(
        &self,
        user_id: UserId,
        ttl: Duration,
        kind: TokenKind,
        repo: &dyn TokenRepo,
        clock: &dyn Clock,
    ) -> Result<AdminToken, TokenError>;

    async fn expire_token(&self, token: AdminToken, repo: &dyn TokenRepo)
    -> Result<(), TokenError>;

    async fn validate_token(
        &self,
        token: AdminToken,
        repo: &dyn TokenRepo,
        clock: &dyn Clock,
    ) -> Result<UserId, TokenError>;
}
