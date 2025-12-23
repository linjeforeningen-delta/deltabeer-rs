mod doc;
pub use doc::ApiDoc;

use axum::{
    Router,
    extract::{Json as JsonIn, Path, State},
    routing::{get, post},
};

use super::types::*;
use crate::api::response::ApiResult;
use crate::state::AppState;

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
async fn resolve_user(
    State(state): State<AppState>,
    Path(ident): Path<String>,
) -> ApiResult<UserIdDto> {
    todo!()
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
    todo!()
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
    todo!()
}
