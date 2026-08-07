mod doc;
pub use doc::ApiDoc;

use super::types::*;
use crate::api::error::ApiError;
use crate::api::response::ApiResult;
use crate::http::v1::types::UserDto;
use crate::state::AppState;
use axum::{
    Extension,
    Json,
    Router,
    body::Body,
    extract::{Json as JsonIn, Path, State},
    http::{Request, StatusCode},
    middleware,
    middleware::Next,
    response::Response,
    routing::{get, patch, post},
};

use delta_core::domain::{Amount, UserId, UserIdent};
use delta_core::services;
use delta_core::services::auth::AdminToken;

pub fn routes(state: AppState) -> Router<AppState> {
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
        .layer(middleware::from_fn_with_state(state, admin_auth_middleware))
}

#[derive(Clone)]
pub struct AdminId(pub UserId);

pub async fn admin_auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
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
    let admin_id = services::auth::validate_authorization(admin_token.clone(), &state.ctx())
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Attach admin identity to request
    req.extensions_mut().insert(AdminId(admin_id));
    req.extensions_mut().insert(admin_token);

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
    let admins = services::users::list_admins(&state.ctx()).await?;
    Ok((
        StatusCode::OK,
        Json(admins.iter().map(UserDto::from).collect()),
    ))
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
    let admin_id = UserId::from(payload.user_id);
    let password = payload.password;
    let token = services::auth::issue_admin_pass(admin_id, password, &state.ctx()).await?;

    Ok((StatusCode::OK, Json(AdminTokenDto::from(&token))))
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
    let token = services::auth::issue_admin_session(admin_id, &state.ctx()).await?;
    Ok((StatusCode::OK, Json(AdminTokenDto::from(&token))))
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
    Extension(AdminId(_admin_id)): Extension<AdminId>,
    Extension(token): Extension<AdminToken>,
) -> ApiResult<()> {
    state
        .ctx()
        .token_repo
        .expire_token(&token)
        .await
        .map_err(delta_core::services::ServiceError::from)?;
    Ok((StatusCode::OK, Json(())))
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
    let card_number = payload
        .card_number
        .parse::<u32>()
        .map_err(|_| ApiError::BadRequest("Invalid card number"))?;
    let user_id = services::users::create_user(
        admin_id,
        services::users::CreateUser {
            name: payload.name,
            username: payload.username,
            card_number,
            birthdate: payload.birthdate,
        },
        &state.ctx(),
    )
        .await?;

    let user = services::users::view_user(user_id, &state.ctx()).await?;
    Ok((StatusCode::OK, Json(UserDto::from(&user))))
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
    let user_ident =
        UserIdent::try_from(ident.as_str()).map_err(|_| ApiError::InvalidUserIdentifier)?;
    let user_id = services::users::resolve_user(user_ident, &state.ctx()).await?;
    let card_number = payload
        .card_number
        .as_deref()
        .map(str::parse::<u32>)
        .transpose()
        .map_err(|_| ApiError::BadRequest("Invalid card number"))?;

    services::users::update_user(
        admin_id,
        user_id,
        services::users::UpdateUser {
            name: payload.name,
            username: payload.username,
            card_number,
            comments: payload.comments,
        },
        &state.ctx(),
    )
        .await?;

    let user = services::users::view_user(user_id, &state.ctx()).await?;
    Ok((StatusCode::OK, Json(UserDto::from(&user))))
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
    let user_ident = UserIdent::try_from(ident.as_str())
        .map_err(|_| ApiError::InvalidUserIdentifier)?;
    let user_id = services::users::resolve_user(user_ident, &state.ctx()).await?;
    let transaction = services::transactions::top_up(
        user_id,
        Amount(payload.0),
        admin_id,
        &state.ctx(),
    )
        .await?;
    Ok((StatusCode::OK, Json(TransactionDto::from(&transaction))))
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
