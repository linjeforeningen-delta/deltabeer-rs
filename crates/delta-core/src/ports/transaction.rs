use crate::domain::{Amount, Transaction, UserId};
use crate::ports::{RepoError, UserRepo};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait TransactionRepo: UserRepo {
    async fn spend(
        &self,
        user: UserId,
        amount: Amount,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError>;
    async fn top_up(
        &self,
        user: UserId,
        amount: Amount,
        approved_by: &UserId,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError>;
}
