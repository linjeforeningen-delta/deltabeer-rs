use super::_types::*;
use delta_core::domain::*;

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

impl From<&User> for UserDto {
    fn from(value: &User) -> Self {
        UserDto {
            id: UserIdDto::from(&value.id),
            name: value.name.clone(),
            username: value.username.clone(),
            card_number: value.card_number.clone(),
            role: RoleDto::from(&value.role),
            birthdate: value.birthdate,
            comments: value.comments.clone(),
            balance: value.balance,
            spent: value.spent,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum MappingError {
    #[error("invalid role")]
    InvalidRole,
    #[error("username required")]
    UsernameRequired,
    // ...
}

impl TryFrom<UserDto> for User {
    type Error = MappingError;

    fn try_from(value: UserDto) -> Result<Self, Self::Error> {
        if value.username.trim().is_empty() {
            return Err(MappingError::UsernameRequired);
        }

        Ok(User {
            id: UserId::try_from(value.id).unwrap(),
            name: value.name,
            username: value.username,
            birthdate: value.birthdate,
            role: Role::try_from(value.role).map_err(|_| MappingError::InvalidRole)?,
            comments: value.comments,
            card_number: value.card_number,
            balance: 0,
            spent: 0,
        })
    }
}

impl From<&TransactionId> for TransactionIdDto {
    fn from(value: &TransactionId) -> Self {
        TransactionIdDto(value.0)
    }
}

impl From<&Transaction> for TransactionDto {
    fn from(value: &Transaction) -> Self {
        TransactionDto {
            id: TransactionIdDto::from(&value.id),
            user_id: UserIdDto::from(&value.user_id),
            amount: value.amount,
            timestamp: value.ts,
            requires_approval: value.requires_approval,
            approved_by: value.approved_by.map(|id| UserIdDto::from(&id)),
        }
    }
}
