use crate::domain::Transaction;
use crate::ports::RepoError;
use async_trait::async_trait;

#[async_trait]
pub trait TransactionRepo {
    async fn insert_transaction(&self, tx: Transaction) -> Result<(), RepoError>;
}
