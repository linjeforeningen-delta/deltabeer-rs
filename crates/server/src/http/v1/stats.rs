use axum::{Json, Router, routing::get};
use serde_json::json;

/// Mounts:
///   GET /v1/stats
///   GET /v1/stats/summary
pub fn routes() -> Router {
    Router::new()
        .route("/stats", get(get_stats))
        .nest("/stats", Router::new().route("/summary", get(summary)))
}

async fn get_stats() -> Json<serde_json::Value> {
    Json(json!({ "ok": true }))
}

async fn summary() -> Json<serde_json::Value> {
    Json(json!({ "total_users": 0, "total_spent": 0, "total_balance": 0 }))
}
