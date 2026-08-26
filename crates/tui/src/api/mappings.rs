use crate::model::{
    Amount, Role, Transaction, TransactionId, TransactionKind, TransactionSource, User, UserId,
    UserPatch,
};
use delta_api::{
    RoleDto, TransactionDto, TransactionKindDto, TransactionSourceDto, UserDto, UserIdDto,
    UserPatchDto,
};

pub(crate) fn user_id_from_dto(value: UserIdDto) -> UserId {
    UserId(value.0)
}
pub(crate) fn user_id_to_dto(value: UserId) -> UserIdDto {
    UserIdDto(value.0)
}

pub(crate) fn user_from_dto(value: UserDto) -> User {
    User {
        id: user_id_from_dto(value.id),
        name: value.name,
        username: value.username,
        program: value.program,
        card_number: value.card_number,
        role: match value.role {
            RoleDto::Admin => Role::Admin,
            RoleDto::User => Role::User,
        },
        birthdate: value.birthdate,
        comments: value.comments,
        balance: Amount(value.balance.0),
        spent: Amount(value.spent.0),
    }
}

pub(crate) fn user_patch_to_dto(value: UserPatch) -> UserPatchDto {
    UserPatchDto {
        name: value.name,
        username: value.username,
        program: value.program,
        card_number: value.card_number,
        comments: value.comments,
    }
}

pub(crate) fn transaction_from_dto(value: TransactionDto) -> Transaction {
    Transaction {
        id: TransactionId(value.id.0),
        user_id: user_id_from_dto(value.user_id),
        kind: match value.kind {
            TransactionKindDto::Spend => TransactionKind::Spend,
            TransactionKindDto::TopUp => TransactionKind::TopUp,
        },
        amount: Amount(value.amount.0),
        timestamp: value.timestamp,
        approved_by: value.approved_by.map(user_id_from_dto),
        source: match value.source {
            TransactionSourceDto::Live => TransactionSource::Live,
            TransactionSourceDto::Migration => TransactionSource::Migration,
            TransactionSourceDto::Adjustment => TransactionSource::Adjustment,
        },
    }
}
