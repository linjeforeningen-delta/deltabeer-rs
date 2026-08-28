use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct UserId(pub(crate) Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Role {
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
pub(crate) struct Amount(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Stats {
    pub(crate) total_users: u32,
    pub(crate) total_balance: Amount,
    pub(crate) total_spent: Amount,
    pub(crate) total_transactions: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct User {
    pub(crate) id: UserId,
    pub(crate) name: String,
    pub(crate) username: String,
    pub(crate) program: String,
    pub(crate) card_number: u32,
    pub(crate) role: Role,
    pub(crate) birthdate: NaiveDate,
    pub(crate) comments: String,
    pub(crate) balance: Amount,
    pub(crate) spent: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UserPatch {
    pub(crate) name: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) program: Option<String>,
    pub(crate) card_number: Option<u32>,
    pub(crate) comments: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TransactionId(pub(crate) Uuid);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransactionKind {
    Spend,
    TopUp,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransactionSource {
    Live,
    Migration,
    Adjustment,
}
#[derive(Debug, Clone)]
pub(crate) struct Transaction {
    pub(crate) id: TransactionId,
    pub(crate) user_id: UserId,
    pub(crate) kind: TransactionKind,
    pub(crate) amount: Amount,
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) approved_by: Option<UserId>,
    pub(crate) source: TransactionSource,
}
