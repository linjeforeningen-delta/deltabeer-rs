use crate::api::models::user::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub user_id: UserId,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct AdminToken(pub String);


#[derive(Debug)]
pub struct SessionToken(String);

#[derive(Debug)]
pub struct SingleUseToken(String);


impl From<AdminToken> for SessionToken {
    fn from(token: AdminToken) -> Self {
        Self(token.0)
    }
}

impl From<AdminToken> for SingleUseToken {
    fn from(token: AdminToken) -> Self {
        Self(token.0)
    }
}