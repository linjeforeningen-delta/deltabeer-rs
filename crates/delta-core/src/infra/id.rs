use crate::domain::{AdminGrantId, TransactionId, UserId};
use crate::ports::IdGenerator;
use chrono::{DateTime, Utc};
use uuid::{NoContext, Timestamp, Uuid};

pub struct UuidIdGenerator;

impl IdGenerator for UuidIdGenerator {
    fn generate_user_id(&self, dt: &DateTime<Utc>) -> UserId {
        let ts = Timestamp::from_unix(
            NoContext,
            dt.timestamp() as u64,
            dt.timestamp_subsec_nanos(),
        );
        UserId(Uuid::new_v7(ts))
    }

    fn generate_transaction_id(&self, dt: &DateTime<Utc>) -> TransactionId {
        let ts = Timestamp::from_unix(
            NoContext,
            dt.timestamp() as u64,
            dt.timestamp_subsec_nanos(),
        );
        TransactionId(Uuid::new_v7(ts))
    }

    fn generate_admin_grant_id(&self, dt: &DateTime<Utc>) -> AdminGrantId {
        let ts = Timestamp::from_unix(
            NoContext,
            dt.timestamp() as u64,
            dt.timestamp_subsec_nanos(),
        );
        AdminGrantId(Uuid::new_v7(ts))
    }
}
