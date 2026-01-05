use crate::domain::{hash_password, needs_rehash, verify_password, ActionRecord, UserId};
use crate::ports::repo::{AdminRepo, TokenRepo, UserRepo};
use crate::ports::TokenError;
use crate::services::context::Ctx;
use crate::services::ServiceError;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminToken(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    SingleUse,
    Session,
}

pub struct TokenData {
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub kind: TokenKind,
}

pub async fn login<R>(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: AdminRepo,
{
    let hash = ctx.repo.get_admin(user_id).await?;
    verify_password(&password, &hash)?;
    if needs_rehash(&hash) {
        update_password(user_id, password, ctx).await?;
    }
    Ok(())
}

pub async fn issue_admin_pass<R>(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, ServiceError>
where
    R: TokenRepo + AdminRepo + UserRepo,
{
    login(user_id, password, ctx).await?;
    let ttl = chrono::Duration::seconds(15);
    let token = ctx
        .tokens
        .issue_token(user_id, ttl, TokenKind::SingleUse, ctx.repo, ctx.clock)
        .await?;

    Ok(token)
}

pub async fn issue_admin_session<R>(
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, ServiceError>
where
    R: TokenRepo,
{
    let now = ctx.clock.now();
    let user_id = ctx
        .tokens
        .validate_token(token.clone(), ctx.repo, ctx.clock)
        .await?;
    let new_token = ctx
        .tokens
        .issue_token(
            user_id,
            chrono::Duration::minutes(3),
            TokenKind::Session,
            ctx.repo,
            ctx.clock,
        )
        .await?;
    match ctx.tokens.expire_token(token, ctx.repo).await {
        Ok(()) => {}
        Err(TokenError::InvalidToken) => {
            // benign: token already expired / invalid
        }
        Err(e) => return Err(e.into()),
    }
    Ok(new_token)
}

pub async fn validate_authorization<R>(
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<UserId, ServiceError>
where
    R: TokenRepo,
{
    Ok(ctx
        .tokens
        .validate_token(token, ctx.repo, ctx.clock)
        .await?)
}

pub async fn grant_admin<R>(
    token: AdminToken,
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: TokenRepo + AdminRepo,
{
    let now = ctx.clock.now();
    let actor = validate_authorization(token, ctx).await?;
    let hash = hash_password(&*password);

    let grant_id = ctx.ids.generate_admin_grant_id();

    let record = ActionRecord { actor, at: now };

    ctx.repo
        .grant_admin(grant_id, user_id, hash, record)
        .await?;
    Ok(())
}

pub async fn revoke_admin<R>(
    token: AdminToken,
    user_id: UserId,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: TokenRepo + AdminRepo,
{
    let now = ctx.clock.now();
    let actor = validate_authorization(token, ctx).await?;

    let record = ActionRecord { actor, at: now };
    ctx.repo.revoke_admin(user_id, record).await?;
    Ok(())
}

pub async fn update_password<R>(
    user_id: UserId,
    new_password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: AdminRepo,
{
    let hash = hash_password(&*new_password);

    ctx.repo.update_admin_password(user_id, hash).await?;

    Ok(())
}
