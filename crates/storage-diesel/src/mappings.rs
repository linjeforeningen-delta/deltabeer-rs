use crate::models::{NewTransaction, TransactionRow, UserWithRoleRow};
use delta_core::domain::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Invalid UUID in stored data")]
    InvalidId {
        #[source]
        source: uuid::Error,
    },
    #[error("Stored data is corrupt")]
    CorruptData,
}

impl From<DomainError> for MappingError {
    fn from(e: DomainError) -> Self {
        tracing::error!(
            error = ?e,
            "domain invariant violated while loading from storage"
        );
        MappingError::CorruptData
    }
}

fn parse_id<T>(s: &str) -> Result<T, MappingError>
where
    for<'a> T: TryFrom<&'a str, Error = uuid::Error>,
{
    T::try_from(s).map_err(|e| MappingError::InvalidId { source: e })
}

/// =======================
/// users
/// =======================

impl TryFrom<&UserWithRoleRow> for User {
    type Error = MappingError;
    fn try_from(value: &UserWithRoleRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: parse_id(&value.id)?,
            name: value.name.clone(),
            username: value.username.clone(),
            card_number: value
                .card_number
                .try_into()
                .map_err(|_| MappingError::CorruptData)?,
            role: match value.role.as_str() {
                "admin" => Role::Admin,
                "user" => Role::User,
                _ => return Err(MappingError::CorruptData),
            },
            birthdate: chrono::NaiveDate::parse_from_str(&value.birthdate, "%Y-%m-%d")
                .map_err(|_| MappingError::CorruptData)?,
            comments: value.comments.clone(),
            balance: Amount::try_from(value.balance)?,
            spent: Amount::try_from(value.spent)?,
        })
    }
}

/// =======================
/// transactions
/// =======================

impl TryFrom<&TransactionRow> for Transaction {
    type Error = MappingError;

    fn try_from(value: &TransactionRow) -> Result<Self, Self::Error> {
        match (value.kind.as_str(), value.approved_by.clone()) {
            ("spend", None) => Ok(Transaction::Spend {
                id: parse_id(&value.id)?,
                user_id: parse_id(&value.user_id)?,
                amount: Amount::try_from(value.amount)?,
                ts: chrono::DateTime::from_timestamp(value.created_at, 0)
                    .ok_or(MappingError::CorruptData)?,
            }),
            ("topup", Some(approver_id)) => Ok(Transaction::TopUp {
                id: parse_id(&value.id)?,
                user_id: parse_id(&value.user_id)?,
                amount: Amount::try_from(value.amount)?,
                ts: chrono::DateTime::from_timestamp(value.created_at, 0)
                    .ok_or(MappingError::CorruptData)?,
                approved_by: parse_id(&approver_id)?,
            }),
            _ => Err(MappingError::CorruptData),
        }
    }
}

impl From<Transaction> for NewTransaction {
    fn from(value: Transaction) -> Self {
        match value {
            Transaction::TopUp {
                id,
                user_id,
                amount,
                approved_by,
                ts,
            } => NewTransaction {
                id: id.0.to_string(),
                user_id: user_id.0.to_string(),
                kind: "topup".to_string(),
                amount: amount.0 as i64,
                approved_by: Some(approved_by.0.to_string()),
                created_at: ts.timestamp(),
            },

            Transaction::Spend {
                id,
                user_id,
                amount,
                ts,
            } => NewTransaction {
                id: id.0.to_string(),
                user_id: user_id.0.to_string(),
                kind: "spend".to_string(),
                amount: amount.0 as i64,
                approved_by: None,
                created_at: ts.timestamp(),
            },
        }
    }
}
