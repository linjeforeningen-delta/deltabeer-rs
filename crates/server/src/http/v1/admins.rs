use axum::{
    Json, Router,
    extract::Path,
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
pub fn routes() -> Router {
    Router::new().route("/admins", get(get_admins)).nest(
        "/admins",
        Router::new()
            .route("/login", post(login))
            .route("/logout", post(logout))
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

async fn get_admins() -> Json<serde_json::Value> {
    Json(json!({ "admins": [] }))
}
async fn login() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}
async fn logout() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}
async fn new_user() -> Json<serde_json::Value> {
    Json(json!({ "created": true }))
}
async fn update_user(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "updated_user": ident }))
}
async fn topup(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "topup_for": ident }))
}
async fn update_role(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "role_updated_for": ident }))
}
