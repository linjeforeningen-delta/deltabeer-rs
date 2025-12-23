use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_admins,
        super::pass,
        super::login,
        super::logout,
        super::new_user,
        super::update_user,
        super::topup,
        super::update_role,
    ),
    tags(
        (name = "admins", description = "Admin-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
