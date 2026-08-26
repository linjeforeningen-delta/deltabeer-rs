pub(crate) use delta_api::{AdminTokenDto, Credentials};

#[derive(Debug, Clone)]
pub(crate) struct SessionToken(String);

impl SessionToken {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub(crate) struct SingleUseToken(String);

impl SingleUseToken {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<AdminTokenDto> for SessionToken {
    fn from(token: AdminTokenDto) -> Self {
        Self(token.0)
    }
}

impl From<AdminTokenDto> for SingleUseToken {
    fn from(token: AdminTokenDto) -> Self {
        Self(token.0)
    }
}

impl From<SingleUseToken> for AdminTokenDto {
    fn from(token: SingleUseToken) -> Self {
        Self(token.0)
    }
}

impl From<SessionToken> for AdminTokenDto {
    fn from(token: SessionToken) -> Self {
        Self(token.0)
    }
}
