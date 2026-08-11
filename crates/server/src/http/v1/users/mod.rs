mod doc;
use axum::http::StatusCode;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::error::ApiError;
use crate::api::response::ApiResult;
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Json as JsonIn, Path, State},
    routing::{get, post},
};
use delta_core::domain::{Amount, UserId, UserIdent};
use delta_core::services;
use uuid::Uuid;

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

fn parse_ident(ident: String) -> Option<UserIdent> {
    Some(if let Ok(id) = Uuid::parse_str(&ident) {
        UserIdent::Id(UserId(id))
    } else if let Ok(card_number) = ident.parse::<u32>() {
        UserIdent::Card(card_number)
    } else if ident.is_ascii() {
        UserIdent::Username(ident)
    } else {
        return None;
    })
}

#[utoipa::path(
    get,
    path = "",
    tag = "users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserDto>)
    )
)]
async fn get_users(State(state): State<AppState>) -> ApiResult<Vec<UserDto>> {
    let users = services::users::list_users(&state.ctx()).await?;
    Ok((
        StatusCode::OK,
        Json(users.iter().map(UserDto::from).collect()),
    ))
}

#[utoipa::path(
    get,
    path = "/resolve/{ident}",
    tag = "users",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "User identifier", body = UserIdDto)
    )
)]
#[axum::debug_handler]
async fn resolve_user(
    State(state): State<AppState>,
    Path(ident): Path<String>,
) -> ApiResult<UserIdDto> {
    let user_ident = parse_ident(ident).ok_or(ApiError::InvalidUserIdentifier)?;

    let user_id = services::users::resolve_user(user_ident, &state.ctx()).await?;

    Ok((StatusCode::OK, Json(UserIdDto::from(&user_id))))
}

#[utoipa::path(
    get,
    path = "/{user_id}",
    tag = "users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Get single user", body = UserDto)
    )
)]
async fn get_user(State(state): State<AppState>, Path(user_id): Path<UserIdDto>) -> ApiResult<UserDto> {
    let user = services::users::view_user(UserId::from(user_id), &state.ctx()).await?;

    Ok((StatusCode::OK, Json(UserDto::from(&user))))
}

#[utoipa::path(
    post,
    path = "/{user_id}/spend",
    tag = "users",
    params(
        ("user_id" = String, Path, description = "User ID")
    ),
    request_body = SpendRequestDto,
    responses(
        (status = 200, description = "Spend accepted", body = TransactionDto)
    )
)]
async fn spend(
    State(state): State<AppState>,
    Path(user_id): Path<UserIdDto>,
    JsonIn(payload): JsonIn<SpendRequestDto>,
) -> ApiResult<TransactionDto> {
    let amount = Amount::from(payload);
    let transaction = services::transactions::spend(UserId::from(user_id), amount, &state.ctx()).await?;
    Ok((StatusCode::OK, Json(TransactionDto::from(&transaction))))
}
