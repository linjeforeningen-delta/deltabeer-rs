use chrono::{DateTime, NaiveDate, Utc};
use rusqlite::types::{FromSql, FromSqlError, ToSql, ToSqlOutput, Value, ValueRef};
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

impl ToSql for Role {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(self.to_string())))
    }
}

impl FromSql for Role {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(s) => {
                let s = std::str::from_utf8(s).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                match s {
                    "Admin" => Ok(Role::Admin),
                    "User" => Ok(Role::User),
                    _ => Err(FromSqlError::Other(
                        format!("invalid role value: {}", s).into(),
                    )),
                }
            }
            _ => Err(FromSqlError::InvalidType),
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

impl ToSql for UserId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // Ok(ToSqlOutput::Owned(Value::Text(self.0.to_string())))
        Ok(ToSqlOutput::from(self.0.to_string()))
    }
}
impl FromSql for UserId {
    fn column_result(v: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match v {
            ValueRef::Text(bytes) => {
                let s = std::str::from_utf8(bytes).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                let uuid = Uuid::parse_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Ok(Self(uuid))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub username: String,
    pub card_number: String,
    pub role: Role,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: u32,
    pub spent: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToSql for TransactionId {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        // Ok(ToSqlOutput::Owned(Value::Text(self.0.to_string())))
        Ok(ToSqlOutput::from(self.0.to_string()))
    }
}
impl FromSql for TransactionId {
    fn column_result(v: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match v {
            ValueRef::Text(bytes) => {
                let s = std::str::from_utf8(bytes).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                let uuid = Uuid::parse_str(s).map_err(|e| FromSqlError::Other(Box::new(e)))?;
                Ok(Self(uuid))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: TransactionId,
    pub user_id: UserId,
    pub amount: u32,
    pub ts: DateTime<Utc>,
    pub requires_approval: bool,
    pub approved_by: Option<UserId>,
}
