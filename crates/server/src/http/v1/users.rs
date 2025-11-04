use axum::{
    Json, Router,
    extract::Path,
    routing::{get, post},
};
use serde_json::json;

/// Mounts:
///   GET  /v1/users
///   GET  /v1/users/resolve/{ident}
///   GET  /v1/users/{ident}
///   POST /v1/users/{ident}/spend
///   PATCH /v1/users/{ident}        (if you want user edits here too)
pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/resolve/{ident}", get(resolve_user))
        .nest(
            "/users",
            Router::new()
                .route("/{ident}", get(get_user).patch(update_user))
                .route("/{ident}/spend", post(spend)),
        )
}

async fn get_users() -> Json<serde_json::Value> {
    Json(json!({ "items": [] }))
}

async fn resolve_user(Path(ident): Path<String>) -> Json<serde_json::Value> {
    // return the canonical UUID for ident (uuid | username | card)
    Json(json!({ "id": ident })) // stub
}

async fn get_user(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "id": ident, "username": "example_user", "balance": 10000 }))
}

async fn update_user(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "updated": ident }))
}

async fn spend(Path(ident): Path<String>) -> Json<serde_json::Value> {
    Json(json!({ "spent_for": ident }))
}
