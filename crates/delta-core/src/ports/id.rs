use crate::domain::{AdminGrantId, TransactionId, UserId};

pub(crate) trait IdGenerator {
    fn generate_user_id(&self) -> UserId;
    fn generate_transaction_id(&self) -> TransactionId;
    fn generate_admin_grant_id(&self) -> AdminGrantId;
}
