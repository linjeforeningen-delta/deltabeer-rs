use crate::domain::{Amount, DomainError, Transaction, TransactionId, UserId};
use crate::ports::{Clock, TransactionRepo, UserRepo};
use chrono::{DateTime, Utc};

struct Ctx<'a> {
    pub users: &'a dyn UserRepo,
    pub transactions: &'a dyn TransactionRepo,
    pub clock: &'a dyn Clock,
}

fn spend(user_id: UserId, amount: Amount, ctx: &Ctx<'_>) -> Result<Transaction, DomainError> {
    // Fetch user
    // Check balance
    // Deduct amount
    // Create transaction record

    // NEEDS: TransactionRepo.insert(tx) -> Result<(), RepoError>
    //        UserRepo.get(user_id) -> Result<User, RepoError>
    //        UserRepo.update(user) -> Result<(), RepoError>
    todo!()
}

fn top_up(
    user_id: UserId,
    amount: Amount,
    admin_id: UserId,
    ctx: &Ctx<'_>,
) -> Result<Transaction, DomainError> {
    // Fetch user
    // Check admin
    // Check balance
    // Add amount
    // Create transaction record

    // NEEDS: TransactionRepo.insert(tx) -> Result<(), RepoError>
    //        UserRepo.get(user_id) -> Result<User, RepoError>
    //        UserRepo.update(user) -> Result<(), RepoError>
    todo!()
}
