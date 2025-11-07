// crates/delta-core/src/ports.rs
use crate::domain::*;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("not found")]
    NotFound,

    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),

    #[error(transparent)]
    Pool(#[from] r2d2::Error),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    // optional catch-all when you *explicitly* convert to anyhow
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get(&self, key: &str) -> Result<User, RepoError>;
    async fn create(&self, user: User) -> Result<(), RepoError>;
    async fn update(&self, user: User) -> Result<(), RepoError>;
}
