use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct Amount(pub u32);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPatch {
    pub name: Option<String>,
    pub username: Option<String>,
    pub program: Option<String>,
    pub card_number: Option<u32>,
    pub comments: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateRequest {
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: u32,
    pub birthdate: NaiveDate,
}
