use crate::{
    domain::UserId,
    ports::{Clock, TokenRepo},
    services::auth::{AdminToken, TokenKind},
};
use async_trait::async_trait;
use chrono::Duration;
use thiserror::Error;

/// Failures from token issuance, invalidation, or validation.
#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Failed to issue token")]
    FailedToIssueToken,
}

/// Boundary for token generation and lifecycle operations.
///
/// Implementations coordinate opaque token material with `TokenRepo`; callers
/// do not need to know how tokens are generated or encoded.
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
