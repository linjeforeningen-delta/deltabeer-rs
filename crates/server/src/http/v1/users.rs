use axum::{
    Json, Router,
    extract::{Json as JsonIn, Path, State},
    http::StatusCode,
    routing::{get, post},
};

use super::_types::*;
use crate::api::response::ApiResult;
use crate::state::AppState;

/// Mounts:
///   GET  /v1/users
///   GET  /v1/users/resolve/{ident}
///   GET  /v1/users/{ident}
///   POST /v1/users/{ident}/spend
///   PATCH /v1/users/{ident}        (if you want user edits here too)

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/resolve/{ident}", get(resolve_user))
        .nest(
            "/users",
            Router::new()
                .route("/{ident}", get(get_user))
                .route("/{ident}/spend", post(spend)),
        )
}

macro_rules! mock_user {
    () => {
        UserDto {
            id: UserIdDto::from(&UserId::new()),
            name: String::from("Ada Lovelace"),
            username: String::from("adalov"),
            card_number: String::from("123456"),
            role: RoleDto::Admin,
            birthdate: NaiveDate::from_ymd_opt(1815, 12, 10).unwrap(),
            comments: String::from("Author of Note G"),
            balance: 0,
            spent: 0,
        }
    };
}

async fn get_users(State(state): State<AppState>) -> ApiResult<Vec<UserDto>> {
    todo!("Implement a queryable user list")
}

async fn resolve_user(
    State(state): State<AppState>,
    Path(ident): Path<String>,
) -> ApiResult<UserIdDto> {
    todo!("Implement a user resolver")
}

async fn get_user(State(state): State<AppState>, Path(ident): Path<String>) -> ApiResult<UserDto> {
    todo!("Implement a user resolver")
}

async fn spend(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<SpendRequestDto>,
) -> ApiResult<TransactionDto> {
    todo!("Implement a user spend operation")
}
