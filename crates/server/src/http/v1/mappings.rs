use super::types::*;
use base64::prelude::*;
use delta_core::{domain::*, services::auth::AdminToken};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Spend transaction has unexpected approval")]
    UnexpectedApproval,
    #[error("TopUp transaction is missing approval")]
    MissingApproval,
    #[error("Invalid token encoding")]
    InvalidTokenEncoding,
}

impl From<&UserId> for UserIdDto {
    fn from(value: &UserId) -> Self {
        UserIdDto(value.0)
    }
}

impl From<UserIdDto> for UserId {
    fn from(value: UserIdDto) -> UserId {
        UserId(value.0)
    }
}

impl From<&Role> for RoleDto {
    fn from(value: &Role) -> Self {
        match value {
            Role::Admin => RoleDto::Admin,
            Role::User => RoleDto::User,
        }
    }
}

impl From<RoleDto> for Role {
    fn from(value: RoleDto) -> Role {
        match value {
            RoleDto::Admin => Role::Admin,
            RoleDto::User => Role::User,
        }
    }
}

impl From<&Amount> for AmountDto {
    fn from(value: &Amount) -> Self {
        AmountDto(value.0)
    }
}

impl From<AmountDto> for Amount {
    fn from(value: AmountDto) -> Self {
        Amount(value.0)
    }
}

impl From<&User> for UserDto {
    fn from(value: &User) -> Self {
        UserDto {
            id: (&value.id).into(),
            name: value.name.clone(),
            username: value.username.clone(),
            card_number: value.card_number.clone(),
            role: (&value.role).into(),
            birthdate: value.birthdate,
            comments: value.comments.clone(),
            balance: (&value.balance).into(),
            spent: (&value.spent).into(),
        }
    }
}

impl From<UserDto> for User {
    fn from(value: UserDto) -> Self {
        User {
            id: value.id.into(),
            name: value.name,
            username: value.username,
            birthdate: value.birthdate,
            role: value.role.into(),
            comments: value.comments,
            card_number: value.card_number,
            balance: value.balance.into(),
            spent: value.spent.into(),
        }
    }
}

impl From<&TransactionId> for TransactionIdDto {
    fn from(value: &TransactionId) -> Self {
        TransactionIdDto(value.0)
    }
}

impl From<TransactionIdDto> for TransactionId {
    fn from(value: TransactionIdDto) -> Self {
        TransactionId(value.0)
    }
}

impl From<&Transaction> for TransactionDto {
    fn from(value: &Transaction) -> Self {
        match value {
            Transaction::Spend {
                id,
                user_id,
                amount,
                ts,
            } => TransactionDto {
                id: id.into(),
                user_id: user_id.into(),
                kind: TransactionKindDto::Spend,
                amount: amount.into(),
                timestamp: *ts,
                approved_by: None,
            },
            Transaction::TopUp {
                id,
                user_id,
                amount,
                ts,
                approved_by,
            } => TransactionDto {
                id: id.into(),
                user_id: user_id.into(),
                kind: TransactionKindDto::TopUp,
                amount: amount.into(),
                timestamp: *ts,
                approved_by: Some(approved_by.into()),
            },
        }
    }
}

impl TryFrom<TransactionDto> for Transaction {
    type Error = MappingError;

    fn try_from(dto: TransactionDto) -> Result<Self, Self::Error> {
        match (dto.kind, dto.approved_by) {
            (TransactionKindDto::Spend, None) => Ok(Transaction::Spend {
                id: dto.id.into(),
                user_id: dto.user_id.into(),
                amount: dto.amount.into(),
                ts: dto.timestamp,
            }),

            (TransactionKindDto::TopUp, Some(approved_by)) => Ok(Transaction::TopUp {
                id: dto.id.into(),
                user_id: dto.user_id.into(),
                amount: dto.amount.into(),
                ts: dto.timestamp,
                approved_by: approved_by.into(),
            }),

            (TransactionKindDto::Spend, Some(_)) => Err(MappingError::UnexpectedApproval),

            (TransactionKindDto::TopUp, None) => Err(MappingError::MissingApproval),
        }
    }
}

impl From<&AdminToken> for AdminTokenDto {
    fn from(value: &AdminToken) -> Self {
        AdminTokenDto(BASE64_URL_SAFE_NO_PAD.encode(value.0))
    }
}

impl TryFrom<AdminTokenDto> for AdminToken {
    type Error = MappingError;

    fn try_from(value: AdminTokenDto) -> Result<Self, Self::Error> {
        let bytes = BASE64_URL_SAFE_NO_PAD
            .decode(value.0)
            .map_err(|_| MappingError::InvalidTokenEncoding)?;
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| MappingError::InvalidTokenEncoding)?;
        Ok(AdminToken(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_token_dto_mapping() {
        let token = AdminToken([42; 32]);
        let dto: AdminTokenDto = (&token).into();

        // Check if it's base64 url safe no pad
        // [42; 32] in base64 is "KioqKioqKioqKioqKioqKioqKioqKioqKioqKioqKio"
        assert_eq!(dto.0, "KioqKioqKioqKioqKioqKioqKioqKioqKioqKioqKio");

        let decoded: AdminToken = dto.try_into().unwrap();
        assert_eq!(decoded, token);
    }
}
