use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsSummary {
    pub total_users: u32,
    pub total_balance: u32,
    pub total_spent: u32,
    pub total_transactions: u32,
}