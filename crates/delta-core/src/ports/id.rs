use crate::domain::{AdminGrantId, TransactionId, UserId};
use chrono::{DateTime, Utc};

pub trait IdGenerator: Send + Sync {
    fn generate_user_id(&self, dt: &DateTime<Utc>) -> UserId;
    fn generate_transaction_id(&self, dt: &DateTime<Utc>) -> TransactionId;
    fn generate_admin_grant_id(&self, dt: &DateTime<Utc>) -> AdminGrantId;
}
