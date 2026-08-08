use crate::domain::{Amount, DomainError};
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

impl TryFrom<&str> for UserId {
    type Error = uuid::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value).map(Self)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum UserIdent {
    Id(UserId),
    Card(u32),
    Username(String),
}

impl<'a> TryFrom<&'a str> for UserIdent {
    type Error = DomainError;

    fn try_from(ident: &'a str) -> Result<Self, Self::Error> {
        let s = ident.trim();

        // UUID
        if let Ok(uuid) = Uuid::parse_str(s) {
            return Ok(UserIdent::Id(UserId(uuid)));
        }

        // Card number (digits only)
        if s.chars().all(|c| c.is_ascii_digit()) {
            return Ok(UserIdent::Card(
                s.parse::<u32>().map_err(|_| DomainError::InvalidIdent)?,
            ));
        }

        // Username (letters only)
        if s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Ok(UserIdent::Username(s.to_string()));
        }

        Err(DomainError::InvalidIdent)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
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

    pub fn deduct_balance(&self, amount: Amount) -> Result<User, DomainError> {
        let new_balance = self.balance.checked_sub(amount)?;

        Ok(User {
            balance: new_balance,
            spent: self.spent + amount,
            ..self.clone()
        })
    }

    pub fn add_balance(&self, amount: Amount) -> Result<User, DomainError> {
        let new_balance = self.balance + amount;
        Ok(User {
            balance: new_balance,
            ..self.clone()
        })
    }
}
