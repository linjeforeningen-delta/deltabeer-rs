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

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AmountDto(pub u32);

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: UserIdDto,
    #[schema(example = "Ada Lovelace")]
    pub name: String,
    #[schema(example = "adalov")]
    pub username: String,
    #[schema(example = 123456)]
    pub card_number: u32,
    pub role: RoleDto,
    #[schema(format = "date", example = "1815-12-10")]
    pub birthdate: NaiveDate,
    #[schema(value_type = String, example = "Author of Note G")]
    pub comments: String,
    pub balance: AmountDto,
    pub spent: AmountDto,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserPatchDto {
    #[schema(nullable, example = "Ada Lovelace")]
    pub name: Option<String>,
    #[schema(nullable, example = "adalov")]
    pub username: Option<String>,
    #[schema(nullable, example = "123456")]
    pub card_number: Option<String>,
    #[schema(nullable, example = "Author of Note G")]
    pub comments: Option<String>,
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
pub enum TransactionKindDto {
    Spend,
    TopUp,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    pub id: TransactionIdDto,
    pub user_id: UserIdDto,
    pub kind: TransactionKindDto,
    pub amount: AmountDto,
    pub timestamp: DateTime<Utc>,
    pub approved_by: Option<UserIdDto>,
}

#[derive(Deserialize, ToSchema)]
#[serde(transparent)]
pub struct SpendRequestDto(pub u32);

#[derive(Deserialize, ToSchema)]
#[serde(transparent)]
pub struct TopupRequestDto(pub u32);

#[derive(Serialize, ToSchema)]
#[serde(transparent)]
pub struct UserIdentificationDto(pub String);

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsDto {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsSummaryDto {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
    pub total_transactions: u32,
}

#[derive(Serialize, ToSchema)]
#[serde(transparent)]
#[schema(example = "SGl0aGVyZUl0c0FCYXNlNjRVcmxTYWZlU3RyaW5n")]
pub struct AdminTokenDto(pub String);

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_id: UserIdDto,
    #[schema(example = "s3cr3tP4ssw0rd")]
    pub password: String,
}
