use crate::domain::Amount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stats {
    pub total_users: u32,
    pub total_balance: Amount,
    pub total_spent: Amount,
    pub total_transactions: u32,
}
