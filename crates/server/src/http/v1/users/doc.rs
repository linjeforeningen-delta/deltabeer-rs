use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_users,
        super::resolve_user,
        super::get_user,
        super::spend,
    ),
    tags(
        (name = "users", description = "User-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
