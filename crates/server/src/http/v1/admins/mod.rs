mod doc;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::response::ApiResult;
use crate::http::v1::types::UserDto;
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
                    .route("/{ident}/role", patch(update_role)),
            ),
    )
}

#[utoipa::path(
    get,
    path = "",
    tag = "admins",
    responses(
        (status = 200, description = "List of admins", body = Vec<UserDto>)
    )
)]
async fn get_admins(State(state): State<AppState>) -> ApiResult<Vec<UserDto>> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/session",
    tag = "admins",
    responses(
        (status = 200, description = "Login response", body = LoginResponse, example="Logged in successfully. Token: ")
    )
)]
async fn login(State(state): State<AppState>) -> ApiResult<LoginResponse> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/session",
    tag = "admins",
    responses(
        (status = 200, description = "Logout response", body = LoginResponse, example="Logged out successfully.")
    )
)]
async fn logout(State(state): State<AppState>) -> ApiResult<LoginResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/user_management/create",
    tag = "admins",
    request_body = UserCreateRequestDto,
    responses(
        (status = 200, description = "Created user", body = UserDto)
    )
)]
async fn new_user(
    State(state): State<AppState>,
    JsonIn(payload): JsonIn<UserCreateRequestDto>,
) -> ApiResult<UserDto> {
    todo!()
}

#[utoipa::path(
    patch,
    path = "/user_management/{ident}/update",
    tag = "admins",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    request_body = UserPatchDto,
    responses(
        (status = 200, description = "Updated user", body = UserDto)
    )
)]
async fn update_user(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<UserPatchDto>,
) -> ApiResult<UserDto> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/user_management/{ident}/topup",
    tag = "admins",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    request_body = TopupRequestDto,
    responses(
        (status = 200, description = "Topup accepted", body = TransactionDto)
    )
)]
async fn topup(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<TopupRequestDto>,
) -> ApiResult<TransactionDto> {
    todo!()
}

#[utoipa::path(
    patch,
    path = "/user_management/{ident}/role",
    tag = "admins",
    params(
        ("ident" = String, Path, description = "User identifier")
    ),
    request_body = RoleDto,
    responses(
        (status = 200, description = "Updated user role", body = UserDto)
    )
)]
async fn update_role(
    State(state): State<AppState>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<RoleDto>,
) -> ApiResult<UserDto> {
    todo!()
}
