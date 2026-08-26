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
        for<'a> T: TryFrom<&'a str, Error=uuid::Error>,
{
    T::try_from(s).map_err(|e| MappingError::InvalidId { source: e })
}

impl TryFrom<&UserWithRoleRow> for User {
    type Error = MappingError;
    fn try_from(value: &UserWithRoleRow) -> Result<Self, Self::Error> {
        Ok(User {
            id: parse_id(&value.id)?,
            name: value.name.clone(),
            username: value.username.clone(),
            program: value.program.clone(),
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

impl TryFrom<&TransactionRow> for Transaction {
    type Error = MappingError;

    fn try_from(value: &TransactionRow) -> Result<Self, Self::Error> {
        let source = TransactionSource::try_from(value.source.as_str())
            .map_err(|_| MappingError::CorruptData)?;

        match (value.kind.as_str(), value.approved_by.clone()) {
            ("spend", None) => Ok(Transaction::Spend {
                id: parse_id(&value.id)?,
                user_id: parse_id(&value.user_id)?,
                amount: Amount::try_from(value.amount)?,
                ts: chrono::DateTime::from_timestamp(value.created_at, 0)
                    .ok_or(MappingError::CorruptData)?,
                source,
            }),
            ("topup", Some(approver_id)) => Ok(Transaction::TopUp {
                id: parse_id(&value.id)?,
                user_id: parse_id(&value.user_id)?,
                amount: Amount::try_from(value.amount)?,
                ts: chrono::DateTime::from_timestamp(value.created_at, 0)
                    .ok_or(MappingError::CorruptData)?,
                approved_by: parse_id(&approver_id)?,
                source,
            }),
            _ => Err(MappingError::CorruptData),
        }
    }
}

impl From<&Transaction> for NewTransaction {
    fn from(value: &Transaction) -> Self {
        match value {
            Transaction::TopUp {
                id,
                user_id,
                amount,
                approved_by,
                ts,
                source,
            } => NewTransaction {
                id: id.0.to_string(),
                user_id: user_id.0.to_string(),
                kind: "topup".to_string(),
                amount: amount.0 as i64,
                source: source.as_str().to_string(),
                approved_by: Some(approved_by.0.to_string()),
                created_at: ts.timestamp(),
            },

            Transaction::Spend {
                id,
                user_id,
                amount,
                ts,
                source,
            } => NewTransaction {
                id: id.0.to_string(),
                user_id: user_id.0.to_string(),
                kind: "spend".to_string(),
                amount: amount.0 as i64,
                source: source.as_str().to_string(),
                approved_by: None,
                created_at: ts.timestamp(),
            },
        }
    }
}
