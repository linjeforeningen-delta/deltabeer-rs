use crate::domain::UserId;
use crate::ports::{Clock, RepoError, TokenError, TokenRepo, TokenSource};
use crate::services::auth::{AdminToken, TokenData, TokenKind};
use async_trait::async_trait;
use chrono::Duration;
use rand_core::{OsRng, RngCore};

impl From<RepoError> for TokenError {
    fn from(_: RepoError) -> Self {
        TokenError::InvalidToken
    }
}

pub struct OpaqueTokenSource;

fn generate_token() -> AdminToken {
    let mut buf = [0u8; 32];
    OsRng.fill_bytes(&mut buf);
    AdminToken(buf)
}

#[async_trait(?Send)]
impl TokenSource for OpaqueTokenSource {
    async fn issue_token(
        &self,
        user_id: UserId,
        ttl: Duration,
        kind: TokenKind,
        repo: &dyn TokenRepo,
        clock: &dyn Clock,
    ) -> Result<AdminToken, TokenError> {
        let now = clock.now();
        let token = generate_token();
        let data = TokenData {
            user_id,
            expires_at: now + ttl,
            kind,
        };

        repo.insert_token(token.clone(), data, now)
            .await
            .map_err(|_| TokenError::FailedToIssueToken)?;
        Ok(token)
    }

    async fn expire_token(
        &self,
        token: AdminToken,
        repo: &dyn TokenRepo,
    ) -> Result<(), TokenError> {
        repo.expire_token(&token)
            .await
            .map_err(|_| TokenError::InvalidToken)?;
        Ok(())
    }

    async fn validate_token(
        &self,
        token: AdminToken,
        repo: &dyn TokenRepo,
        clock: &dyn Clock,
    ) -> Result<UserId, TokenError> {
        let now = clock.now();
        let data = repo
            .get_token(&token, now)
            .await?
            .ok_or(TokenError::InvalidToken)?;

        if data.kind == TokenKind::SingleUse {
            self.expire_token(token, repo).await?;
        }

        let user_id = data.user_id;
        Ok(user_id)
    }
}
