//! Adapters between the HTTP contract and the backend domain.

use base64::prelude::*;
use delta_api::{
    AdminTokenDto, AmountDto, RoleDto, SpendRequestDto, TransactionDto, TransactionIdDto,
    TransactionKindDto, TransactionSourceDto, UserDto, UserIdDto, UserPatchDto,
};
use delta_core::{domain::*, services::auth::AdminToken};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MappingError {
    #[error("Invalid token encoding")]
    InvalidTokenEncoding,
}

pub fn user_id_to_dto(value: &UserId) -> UserIdDto {
    UserIdDto(value.0)
}
pub fn user_id_from_dto(value: UserIdDto) -> UserId {
    UserId(value.0)
}

pub fn role_to_dto(value: &Role) -> RoleDto {
    match value {
        Role::Admin => RoleDto::Admin,
        Role::User => RoleDto::User,
    }
}
pub fn amount_to_dto(value: &Amount) -> AmountDto {
    AmountDto(value.0)
}
pub fn amount_from_spend_dto(value: SpendRequestDto) -> Amount {
    Amount(value.0)
}

pub fn user_to_dto(value: &User) -> UserDto {
    UserDto {
        id: user_id_to_dto(&value.id),
        name: value.name.clone(),
        username: value.username.clone(),
        program: value.program.clone(),
        card_number: value.card_number,
        role: role_to_dto(&value.role),
        birthdate: value.birthdate,
        comments: value.comments.clone(),
        balance: amount_to_dto(&value.balance),
        spent: amount_to_dto(&value.spent),
    }
}

pub fn user_patch_from_dto(value: UserPatchDto) -> delta_core::services::users::UpdateUser {
    delta_core::services::users::UpdateUser {
        name: value.name,
        username: value.username,
        program: value.program,
        card_number: value.card_number,
        comments: value.comments,
    }
}

pub fn transaction_to_dto(value: &Transaction) -> TransactionDto {
    match value {
        Transaction::Spend {
            id,
            user_id,
            amount,
            ts,
            source,
        } => TransactionDto {
            id: TransactionIdDto(id.0),
            user_id: user_id_to_dto(user_id),
            kind: TransactionKindDto::Spend,
            amount: amount_to_dto(amount),
            timestamp: *ts,
            approved_by: None,
            source: source_to_dto(*source),
        },
        Transaction::TopUp {
            id,
            user_id,
            amount,
            ts,
            approved_by,
            source,
        } => TransactionDto {
            id: TransactionIdDto(id.0),
            user_id: user_id_to_dto(user_id),
            kind: TransactionKindDto::TopUp,
            amount: amount_to_dto(amount),
            timestamp: *ts,
            approved_by: Some(user_id_to_dto(approved_by)),
            source: source_to_dto(*source),
        },
    }
}

fn source_to_dto(value: TransactionSource) -> TransactionSourceDto {
    match value {
        TransactionSource::Live => TransactionSourceDto::Live,
        TransactionSource::Migration => TransactionSourceDto::Migration,
        TransactionSource::Adjustment => TransactionSourceDto::Adjustment,
    }
}

pub fn admin_token_to_dto(value: &AdminToken) -> AdminTokenDto {
    AdminTokenDto(BASE64_URL_SAFE_NO_PAD.encode(value.0))
}

pub fn admin_token_from_dto(value: AdminTokenDto) -> Result<AdminToken, MappingError> {
    let bytes = BASE64_URL_SAFE_NO_PAD
        .decode(value.0)
        .map_err(|_| MappingError::InvalidTokenEncoding)?;
    let bytes: [u8; 32] = bytes
        .try_into()
        .map_err(|_| MappingError::InvalidTokenEncoding)?;
    Ok(AdminToken(bytes))
}
