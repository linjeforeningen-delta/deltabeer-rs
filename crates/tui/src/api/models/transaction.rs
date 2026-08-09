use crate::api::models::user::{Amount, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct TransactionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionKind {
    Spend,
    TopUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionSource {
    Live,
    Migration,
    Adjustment,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: TransactionId,
    pub user_id: UserId,
    pub kind: TransactionKind,
    pub amount: Amount,
    pub timestamp: DateTime<Utc>,
    pub approved_by: Option<UserId>,
    pub source: TransactionSource,
}

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct SpendRequest(pub u32);

#[derive(Debug, Serialize)]
#[serde(transparent)]
pub struct TopupRequest(pub u32);