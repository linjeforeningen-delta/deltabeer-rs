use crate::api::{error::ApiError, response::ApiResult};
use crate::state::AppState;
use axum::{Json, Router, http::StatusCode, routing::get};
use serde::Serialize;
use tower_http::trace::TraceLayer;
use utoipa::{OpenApi, ToSchema};

use utoipa_swagger_ui::SwaggerUi;

pub(crate) mod v1;

use crate::http::v1::ApiDoc as V1ApiDoc;

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        // global (non-versioned) health
        .route("/health", get(health))
        // versioned API
        .nest("/v1", v1::routes(state.clone()))
        .merge(
            SwaggerUi::new("/docs") // UI at /docs
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .layer(TraceLayer::new_for_http())
        .fallback(|| async { ApiError::NotFound("404") })
}

#[derive(Serialize, ToSchema)]
struct HealthResponse {
    ok: bool,
    #[schema(example = "0.1.0")]
    version: &'static str,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Health check", body = HealthResponse)
    )
)]
async fn health() -> ApiResult<HealthResponse> {
    Ok((
        StatusCode::OK,
        Json(HealthResponse {
            ok: true,
            version: env!("CARGO_PKG_VERSION"),
        }),
    ))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        health,
    ),
    tags(
        (name = "health", description = "Health check endpoints")
    ),
    nest(
        (path = "/v1", api = V1ApiDoc))
)
]
pub struct ApiDoc;
