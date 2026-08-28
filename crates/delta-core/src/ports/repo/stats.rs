use crate::{domain::Stats, ports::repo::RepoError};
use async_trait::async_trait;

#[async_trait]
pub trait StatsRepo {
    async fn stats(&self) -> Result<Stats, RepoError>;
}
