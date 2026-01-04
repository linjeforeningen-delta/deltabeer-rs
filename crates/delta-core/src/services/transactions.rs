use crate::domain::{Amount, Transaction, UserId};
use crate::ports::{RepoError, TokenRepo, TransactionRepo, UserRepo};
use crate::services::auth::validate_authorization;
use crate::services::context::Ctx;
use crate::services::{auth::AdminToken, ServiceError};

const MAX_RETRIES: usize = 3;
pub async fn spend<R>(
    user_id: UserId,
    amount: Amount,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo,
{
    for _ in 0..MAX_RETRIES {
        match ctx.repo.spend(user_id, amount, ctx.clock.now()).await {
            Ok(tx) => {
                return Ok(tx);
            }
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}

pub async fn top_up<R>(
    user_id: UserId,
    amount: Amount,
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo + TokenRepo,
{
    let admin_id = validate_authorization(token, ctx)?;

    for _ in 0..MAX_RETRIES {
        match ctx
            .repo
            .top_up(user_id, amount, &admin_id, ctx.clock.now())
            .await
        {
            Ok(tx) => {
                return Ok(tx);
            }
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}
