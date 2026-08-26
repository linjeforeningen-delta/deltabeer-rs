use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String, format = "uuid", example = "c56a4180-65aa-42ec-a945-5fd21dec0538")]
pub struct UserIdDto(pub Uuid);

impl fmt::Display for UserIdDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum RoleDto {
    Admin,
    User,
}

impl fmt::Display for RoleDto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Admin => "Admin",
            Self::User => "User",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct AmountDto(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: UserIdDto,
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: u32,
    pub role: RoleDto,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: AmountDto,
    pub spent: AmountDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPatchDto {
    pub name: Option<String>,
    pub username: Option<String>,
    pub program: Option<String>,
    pub card_number: Option<u32>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateRequestDto {
    pub name: String,
    pub username: String,
    pub program: String,
    pub card_number: u32,
    pub birthdate: NaiveDate,
}
