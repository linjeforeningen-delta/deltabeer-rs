use super::user::{AmountDto, UserIdDto};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
#[schema(value_type = String, format = "uuid", example = "c56a4180-65aa-42ec-a945-5fd21dec0538")]
pub struct TransactionIdDto(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TransactionKindDto {
    Spend,
    TopUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum TransactionSourceDto {
    Live,
    Migration,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDto {
    pub id: TransactionIdDto,
    pub user_id: UserIdDto,
    pub kind: TransactionKindDto,
    pub amount: AmountDto,
    pub timestamp: DateTime<Utc>,
    pub approved_by: Option<UserIdDto>,
    pub source: TransactionSourceDto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct SpendRequestDto(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct TopupRequestDto(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct UserIdentificationDto(pub String);
