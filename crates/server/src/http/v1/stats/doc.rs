use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_stats,
        super::summary,
    ),
    tags(
        (name = "stats", description = "Stats-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
