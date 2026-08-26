use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsDto {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatsSummaryDto {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
    pub total_transactions: u32,
}
