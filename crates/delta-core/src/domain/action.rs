use crate::domain::user::UserId;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActionRecord {
    pub actor: UserId,
    pub at: DateTime<Utc>,
}
