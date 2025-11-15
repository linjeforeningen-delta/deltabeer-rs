use utoipa::OpenApi;

use crate::api::error::ErrorBody;

use super::{
    LoginResponse, SpendRequestDto, TopupRequestDto, TransactionDto, UserCreateRequestDto, UserDto,
    UserPatchDto,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        super::get_admins,
        super::login,
        super::logout,
        super::new_user,
        super::update_user,
        super::topup,
        super::update_role,
    ),
    components(
        schemas(
            // ErrorBody,
            // UserDto,
            // UserCreateRequestDto,
        )
    ),
    tags(
        (name = "users", description = "User-related endpoints"),
    ),
    servers((url = "/v1"))
)]
pub struct ApiDoc;
