use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Admin => "Admin",
            Self::User => "User",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Amount(pub u32);

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: u32,
    pub role: Role,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: Amount,
    pub spent: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserPatch {
    pub name: Option<String>,
    pub username: Option<String>,
    pub program: Option<String>,
    pub card_number: Option<u32>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransactionId(pub Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionKind {
    Spend,
    TopUp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionSource {
    Live,
    Migration,
    Adjustment,
}
#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub user_id: UserId,
    pub kind: TransactionKind,
    pub amount: Amount,
    pub timestamp: DateTime<Utc>,
    pub approved_by: Option<UserId>,
    pub source: TransactionSource,
}
