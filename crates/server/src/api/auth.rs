use super::user::UserIdDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct AdminTokenDto(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_id: UserIdDto,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(transparent)]
pub struct PasswordDto(pub String);
