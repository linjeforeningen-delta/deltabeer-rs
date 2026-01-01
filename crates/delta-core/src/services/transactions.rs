use crate::domain::{Amount, Transaction, TransactionId, UserId};
use crate::ports::RepoError;
use crate::services::context::{HasClock, HasTokens, HasTransactions, HasUsers};
use crate::services::{
    auth::{validate_authorization, AdminToken},
    ServiceError,
};

const MAX_RETRIES: usize = 3;
pub async fn spend<T>(user_id: UserId, amount: Amount, ctx: &T) -> Result<Transaction, ServiceError>
where
    T: HasTransactions + HasUsers + HasClock,
{
    let user = ctx.users().get(&user_id).await?.deduct_balance(amount)?;

    // make this atomic
    for _ in 0..MAX_RETRIES {
        let tx = Transaction::Spend {
            id: TransactionId::new(),
            user_id,
            amount,
            ts: ctx.clock().now(),
        };
        match ctx.transactions().insert(tx.clone()).await {
            Ok(()) => {
                ctx.users().update(user).await?;
                return Ok(tx);
            }
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}

pub async fn top_up<T>(
    user_id: UserId,
    amount: Amount,
    token: AdminToken,
    ctx: &T,
) -> Result<Transaction, ServiceError>
where
    T: HasTransactions + HasUsers + HasClock + HasTokens,
{
    let user = ctx.users().get(&user_id).await?.add_balance(amount)?;
    let admin_id = validate_authorization(token, ctx)?;

    // make this atomic
    for _ in 0..MAX_RETRIES {
        let tx = Transaction::TopUp {
            id: TransactionId::new(),
            user_id,
            amount,
            ts: ctx.clock().now(),
            approved_by: admin_id,
        };
        match ctx.transactions().insert(tx.clone()).await {
            Ok(()) => {
                ctx.users().update(user).await?;
                return Ok(tx);
            }
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}
