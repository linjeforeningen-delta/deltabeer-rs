use utoipa::OpenApi;

use crate::api::error::ErrorBody;

use super::{StatsDto, StatsSummaryDto};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_stats,
        super::summary,
    ),
    components(
        schemas(
            // ErrorBody,
            // StatsDto,
            // StatsSummaryDto,
        )
    ),
    tags(
        (name = "stats", description = "Stats-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
