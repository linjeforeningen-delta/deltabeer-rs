use crate::domain::{Amount, Transaction, TransactionId, UserId};
use crate::ports::{RepoError, TokenRepo, TransactionRepo, UserRepo};
use crate::services::context::Ctx;
use crate::services::{
    auth::{validate_authorization, AdminToken},
    ServiceError,
};

const MAX_RETRIES: usize = 3;
pub async fn spend<R>(
    user_id: UserId,
    amount: Amount,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo,
{
    let user = ctx.repo.get_user(&user_id).await?.deduct_balance(amount)?;

    // make this atomic
    for _ in 0..MAX_RETRIES {
        let tx = Transaction::Spend {
            id: TransactionId::new(),
            user_id,
            amount,
            ts: ctx.clock.now(),
        };
        match ctx.repo.insert_transaction(tx.clone()).await {
            Ok(()) => {
                ctx.repo.update_user(user).await?;
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
    let user = ctx.repo.get_user(&user_id).await?.add_balance(amount)?;
    let admin_id = validate_authorization(token, ctx)?;

    // make this atomic
    for _ in 0..MAX_RETRIES {
        let tx = Transaction::TopUp {
            id: TransactionId::new(),
            user_id,
            amount,
            ts: ctx.clock.now(),
            approved_by: admin_id,
        };
        match ctx.repo.insert_transaction(tx.clone()).await {
            Ok(()) => {
                ctx.repo.update_user(user).await?;
                return Ok(tx);
            }
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}
