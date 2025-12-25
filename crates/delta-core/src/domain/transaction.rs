use crate::domain::user::UserId;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum TransactionKind {
    Spend,
    TopUp,
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Approval {
    NotRequired,
    Approved { by: UserId },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Amount(pub u32);

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Transaction {
    Spend {
        id: TransactionId,
        user_id: UserId,
        amount: Amount,
        ts: DateTime<Utc>,
    },
    TopUp {
        id: TransactionId,
        user_id: UserId,
        amount: Amount,
        ts: DateTime<Utc>,
        approved_by: UserId,
    },
}

impl TransactionKind {
    pub fn requires_approval(self) -> bool {
        matches!(self, TransactionKind::TopUp)
    }
}
