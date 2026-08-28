use chrono::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthPolicy {
    pub single_use_token_ttl: Duration,
    pub admin_session_ttl: Duration,
}

impl Default for AuthPolicy {
    fn default() -> Self {
        Self {
            single_use_token_ttl: Duration::seconds(15),
            admin_session_ttl: Duration::minutes(10),
        }
    }
}
