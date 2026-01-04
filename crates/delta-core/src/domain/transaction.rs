use crate::domain::DomainError;
use crate::domain::user::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for TransactionId {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value).map(Self)
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

impl Amount {
    pub fn checked_sub(self, rhs: Amount) -> Result<Amount, DomainError> {
        self.0
            .checked_sub(rhs.0)
            .map(Amount)
            .ok_or(DomainError::InsufficientBalance)
    }
}
impl Add for Amount {
    type Output = Amount;

    fn add(self, rhs: Amount) -> Amount {
        Amount(self.0 + rhs.0)
    }
}
impl TryFrom<i64> for Amount {
    type Error = DomainError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(Amount(value as u32))
        } else {
            Err(DomainError::InvalidAmount)
        }
    }
}

impl From<Amount> for i64 {
    fn from(value: Amount) -> Self {
        value.0 as i64
    }
}

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
