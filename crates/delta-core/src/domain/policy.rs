use chrono::Duration;

/// Durations governing the two admin token kinds.
///
/// The policy is consumed by authentication services when issuing tokens; it
/// does not itself validate or persist them.
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
