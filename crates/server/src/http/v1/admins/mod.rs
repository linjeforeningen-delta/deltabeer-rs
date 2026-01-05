mod doc;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::response::ApiResult;
use crate::http::v1::types::UserDto;
use crate::state::AppState;
use axum::{
    body::Body, extract::{Json as JsonIn, Path, State},
    http::{Request, StatusCode},
    middleware,
    middleware::Next,
    response::Response,
    routing::{get, patch, post},
    Extension,
    Router,
};

use delta_core::domain::UserId;
use delta_core::services::auth::AdminToken;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admins", get(get_admins))
        .nest(
            "/admins",
            Router::new()
                .route("/pass", post(pass))
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
        .layer(middleware::from_fn(admin_auth_middleware))
}

#[derive(Clone)]
pub struct AdminId(pub UserId);

pub async fn admin_auth_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // todo!()
    let path = req.uri().path();

    // Allow unauthenticated access to /admins/pass
    if path == "/admins/pass" {
        return Ok(next.run(req).await);
    }

    // Extract Authorization header
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let token = match auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Wrap token (no validation yet)
    let admin_token: AdminToken = AdminTokenDto(token.to_string())
        .try_into()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Validate token via core
    // let admin_id = state
    //     .services
    //     .auth
    //     .validate_authorization(admin_token)
    //     .await
    //     .map_err(|_| StatusCode::UNAUTHORIZED)?;
    let admin_id = todo!();

    // Attach admin identity to request
    req.extensions_mut().insert(AdminId(admin_id));

    Ok(next.run(req).await)
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
    path = "/pass",
    tag = "admins",
    security(),
    request_body = Credentials,
    responses(
        (status = 200, description = "Login response", body = AdminTokenDto)
    )
)]
async fn pass(
    State(state): State<AppState>,
    JsonIn(payload): JsonIn<Credentials>,
) -> ApiResult<AdminTokenDto> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/session",
    tag = "admins",
    responses(
        (status = 200, description = "Login response", body = AdminTokenDto)
    )
)]
async fn login(
    State(state): State<AppState>,
    Extension(AdminId(admin_id)): Extension<AdminId>,
) -> ApiResult<AdminTokenDto> {
    todo!()
}

#[utoipa::path(
    delete,
    path = "/session",
    tag = "admins",
    responses(
        (status = 200, description = "Logout successful")
    )
)]
async fn logout(
    State(state): State<AppState>,
    Extension(AdminId(admin_id)): Extension<AdminId>,
) -> ApiResult<()> {
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
    Extension(AdminId(admin_id)): Extension<AdminId>,
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
    Extension(AdminId(admin_id)): Extension<AdminId>,
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
    Extension(AdminId(admin_id)): Extension<AdminId>,
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
    Extension(AdminId(admin_id)): Extension<AdminId>,
    Path(ident): Path<String>,
    JsonIn(payload): JsonIn<RoleDto>,
) -> ApiResult<UserDto> {
    todo!()
}
