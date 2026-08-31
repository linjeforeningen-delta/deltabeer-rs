use crate::{domain::Stats, ports::repo::RepoError};
use async_trait::async_trait;

/// Read-only persistence operations for aggregate statistics.
#[async_trait]
pub trait StatsRepo {
    async fn stats(&self) -> Result<Stats, RepoError>;
}
