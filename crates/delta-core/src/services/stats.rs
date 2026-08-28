use crate::{domain::Stats, ports::repo::StatsRepo, services::ServiceError};

pub async fn get_stats<R>(repo: &R) -> Result<Stats, ServiceError>
where
    R: StatsRepo + ?Sized,
{
    Ok(repo.stats().await?)
}
