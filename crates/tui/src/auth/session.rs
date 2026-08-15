use crate::api::models::auth::SessionToken;
use crate::api::models::user::User;
use chrono::{DateTime, Utc};

pub(crate) struct AdminSession {
    admin: User,
    pub(crate) token: SessionToken,
    expires_at: DateTime<Utc>,
    last_validated_at: DateTime<Utc>,
}
