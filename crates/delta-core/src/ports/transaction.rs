use crate::domain::{Transaction, TransactionId};
use crate::ports::RepoError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionRepo {
    async fn insert(&self, tx: Transaction) -> Result<(), RepoError>;
}
