use utoipa::OpenApi;

use crate::api::error::ErrorBody;

use super::{SpendRequestDto, TransactionDto, UserDto};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_users,
        super::resolve_user,
        super::get_user,
        super::spend,
    ),
    components(
        schemas(
            // ErrorBody,
            // UserDto,
            // SpendRequestDto,
            // TransactionDto,
        )
    ),
    tags(
        (name = "users", description = "User-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
