use super::_types::*;
use crate::api::response::ApiResult;
use crate::http::v1::_types::UserDto;
use crate::state::AppState;
use axum::{
    Json, Router,
    extract::{Json as JsonIn, Path, State},
    routing::{get, patch, post, put},
};
use serde_json::json;

/// Mounts:
///   GET  /v1/admins
///   POST /v1/admins/login
///   POST /v1/admins/logout
///   POST /v1/admins/user_management/create
///   PATCH /v1/admins/user_management/{ident}/update
///   POST /v1/admins/user_management/{ident}/topup
///   PUT  /v1/admins/user_management/{ident}/role
pub fn routes() -> Router<AppState> {
    Router::new().route("/admins", get(get_admins)).nest(
        "/admins",
        Router::new()
            .route("/session", post(login).delete(logout))
            .nest(
                "/user_management",
                Router::new()
                    .route("/create", post(new_user))
                    .route("/{ident}/update", patch(update_user))
                    .route("/{ident}/topup", post(topup))
                    .route("/{ident}/role", put(update_role)),
            ),
    )
}

async fn get_admins(State(state): State<AppState>) -> ApiResult<Vec<UserDto>> {
    todo!("Implement a queryable admin list")
}
async fn login(State(state): State<AppState>) -> ApiResult<LoginResponse> {
    todo!("Implement login logic")
}
async fn logout(State(state): State<AppState>) -> ApiResult<LoginResponse> {
    todo!("Implement logout logic")
}
async fn new_user(
    State(state): State<AppState>,
    JsonIn(payload): JsonIn<UserCreateRequestDto>,
) -> ApiResult<UserDto> {
    todo!()
}
async fn update_user(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<UserPatchDto>,
) -> ApiResult<UserDto> {
    todo!()
}
async fn topup(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<TopupRequestDto>,
) -> ApiResult<TransactionDto> {
    todo!()
}
async fn update_role(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<UserRoleUpdateDto>,
) -> ApiResult<UserDto> {
    todo!("Implement a role update")
}
