use crate::{
    domain::{Amount, Transaction, TransactionId, UserId},
    ports::repo::{RepoError, UserRepo},
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};

/// Atomic persistence operations for balance-changing transactions.
///
/// Implementations must persist the balance change and transaction record as
/// one operation so callers do not observe only half of a transaction.
#[async_trait]
pub trait TransactionRepo: UserRepo {
    async fn spend(
        &self,
        tx_id: TransactionId,
        user: UserId,
        amount: Amount,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError>;
    async fn top_up(
        &self,
        tx_id: TransactionId,
        user: UserId,
        amount: Amount,
        approved_by: &UserId,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError>;
}
