use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String, format = "uuid", example = "c56a4180-65aa-42ec-a945-5fd21dec0538")]
pub struct UserIdDto(pub Uuid);

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String, format = "uuid", example = "c56a4180-65aa-42ec-a945-5fd21dec0538")]
pub struct TransactionIdDto(pub Uuid);

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum RoleDto {
    Admin,
    User,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: UserIdDto,
    pub name: String,
    pub username: String,
    pub card_number: String,
    pub role: RoleDto,
    pub birthdate: NaiveDate,
    pub comments: String,
    pub balance: i32,
    pub spent: i32,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPatchDto {
    pub name: Option<String>,
    pub username: Option<String>,
    pub card_number: Option<String>,
    pub comments: Option<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleUpdateDto {
    pub role: RoleDto,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateRequestDto {
    pub name: String,
    pub username: String,
    pub card_number: String,
    pub role: RoleDto,
    pub birthdate: NaiveDate,
    pub comments: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    pub id: TransactionIdDto,
    pub user_id: UserIdDto,
    pub amount: i32,
    pub timestamp: DateTime<Utc>,
    pub requires_approval: bool,
    pub approved_by: Option<UserIdDto>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpendRequestDto {
    pub amount: i32,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TopupRequestDto {
    pub amount: i32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentificationDto {
    pub ident: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsDto {
    pub total_users: usize,
    pub total_balance: i32,
    pub total_spent: i32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsSummaryDto {
    pub total_users: usize,
    pub total_balance: i32,
    pub total_spent: i32,
    pub total_transactions: usize,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub message: String,
}
