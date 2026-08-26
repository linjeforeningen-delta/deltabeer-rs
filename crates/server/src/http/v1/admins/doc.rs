use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);

        components.add_security_scheme(
            "admin_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("opaque")
                    .description(Some("Admin authorization using Bearer token".to_string()))
                    .build(),
            ),
        );
    }
}

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
        super::grant_admin,
        super::revoke_admin,
    ),
    tags(
        (name = "admins", description = "Admin-related endpoints"),
    ),
    modifiers(&SecurityAddon),
    security(
        ("admin_token" = [])
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
