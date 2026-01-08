mod doc;
use axum::http::StatusCode;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::error::ApiError;
use crate::api::response::ApiResult;
use crate::state::AppState;
use axum::response::IntoResponse;
use axum::{
    extract::{Json as JsonIn, Path, State}, routing::{get, post},
    Json,
    Router,
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
    todo!()
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
    path = "/{ident}",
    tag = "users",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    responses(
        (status = 200, description = "Get single user", body = UserDto)
    )
)]
async fn get_user(State(state): State<AppState>, Path(ident): Path<String>) -> ApiResult<UserDto> {
    let user_ident = parse_ident(ident).ok_or(ApiError::InvalidUserIdentifier)?;
    let user_id = services::users::resolve_user(user_ident, &state.ctx()).await?;
    let user = services::users::view_user(user_id, &state.ctx()).await?;

    Ok((StatusCode::OK, Json(UserDto::from(&user))))
}

#[utoipa::path(
    post,
    path = "/{ident}/spend",
    tag = "users",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    request_body = SpendRequestDto,
    responses(
        (status = 200, description = "Spend accepted", body = TransactionDto)
    )
)]
async fn spend(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<SpendRequestDto>,
) -> ApiResult<TransactionDto> {
    let user_ident = parse_ident(ident).ok_or(ApiError::InvalidUserIdentifier)?;
    let user_id = services::users::resolve_user(user_ident, &state.ctx()).await?;
    let amount = Amount::from(payload);
    let transaction = services::transactions::spend(user_id, amount, &state.ctx()).await?;
    Ok((StatusCode::OK, Json(TransactionDto::from(&transaction))))
}
