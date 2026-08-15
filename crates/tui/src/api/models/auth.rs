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

impl AdminToken {
    pub(crate) fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub struct SingleUseToken(String);

impl SingleUseToken {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

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

impl From<SingleUseToken> for AdminToken {
    fn from(token: SingleUseToken) -> Self {
        Self(token.0)
    }
}

impl From<SessionToken> for AdminToken {
    fn from(token: SessionToken) -> Self {
        Self(token.0)
    }
}
