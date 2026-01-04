use crate::domain::{Amount, Transaction, UserId};
use crate::ports::{TokenRepo, TransactionRepo, UserRepo};
use crate::services::auth::validate_authorization;
use crate::services::context::Ctx;
use crate::services::{auth::AdminToken, ServiceError};

pub async fn spend<R>(
    user_id: UserId,
    amount: Amount,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo,
{
    let tx_id = ctx.ids.generate_transaction_id();
    let tx = ctx
        .repo
        .spend(tx_id, user_id, amount, ctx.clock.now())
        .await?;
    Ok(tx)
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
    let tx_id = ctx.ids.generate_transaction_id();

    let tx = ctx
        .repo
        .top_up(tx_id, user_id, amount, &admin_id, ctx.clock.now())
        .await?;
    Ok(tx)
}
