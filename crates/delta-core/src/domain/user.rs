use crate::domain::Amount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Role {
    Admin,
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::User => write!(f, "User"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<&str> for UserId {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value).map(Self)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub username: String,
    pub card_number: u32,
    pub role: Role,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: Amount,
    pub spent: Amount,
}

impl User {
    pub fn is_adult(birthdate: NaiveDate, today: NaiveDate) -> bool {
        birthdate <= today - chrono::Duration::days(18 * 365)
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn has_sufficient_balance(&self, amount: Amount) -> bool {
        self.balance >= amount
    }
}
