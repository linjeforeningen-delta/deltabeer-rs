use crate::domain::{AdminGrantId, TransactionId, UserId};
use crate::ports::IdGenerator;
use uuid::Uuid;

pub struct UuidIdGenerator;

impl IdGenerator for UuidIdGenerator {
    fn generate_user_id(&self) -> UserId {
        UserId(Uuid::now_v7())
    }

    fn generate_transaction_id(&self) -> TransactionId {
        TransactionId(Uuid::now_v7())
    }

    fn generate_admin_grant_id(&self) -> AdminGrantId {
        AdminGrantId(Uuid::now_v7())
    }
}
