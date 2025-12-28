mod mappings;
mod models;
mod schema;

use async_trait::async_trait;
use delta_core::ports::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo};
use thiserror::Error;

use delta_core::domain::{Transaction, User, UserId};
use delta_core::services::auth::{AdminToken, TokenData};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("failed to create database pool")]
    Build(#[from] r2d2::Error),
}

pub fn create_pool(database_url: &str) -> Result<SqlitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .max_size(16)
        .build(manager)
        .map_err(PoolError::from)
}
pub struct DieselRepo {
    pool: SqlitePool,
}

impl DieselRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

#[async_trait]
impl UserRepo for DieselRepo {
    async fn get(&self, key: &str) -> Result<User, RepoError> {
        todo!()
    }

    async fn get_by_name(&self, name: &str) -> Result<User, RepoError> {
        todo!()
    }

    async fn get_by_card(&self, card_number: u32) -> Result<User, RepoError> {
        todo!()
    }

    async fn insert(&self, user: User) -> Result<(), RepoError> {
        todo!()
    }

    async fn update(&self, user: User) -> Result<(), RepoError> {
        todo!()
    }
}

#[async_trait]
impl AdminRepo for DieselRepo {
    async fn get(&self, id: UserId) -> Result<String, RepoError> {
        todo!()
    }

    async fn insert(&self, id: UserId, data: String) -> Result<(), RepoError> {
        todo!()
    }

    async fn remove(&self, id: UserId) -> Result<(), RepoError> {
        todo!()
    }
}

#[async_trait]
impl TransactionRepo for DieselRepo {
    async fn insert(&self, tx: Transaction) -> Result<(), RepoError> {
        todo!()
    }
}

#[async_trait]
impl TokenRepo for DieselRepo {
    async fn insert(&self, token: AdminToken, data: TokenData) -> Result<(), RepoError> {
        todo!()
    }

    async fn get(&self, token: &AdminToken) -> Result<TokenData, RepoError> {
        todo!()
    }

    async fn remove(&self, token: &AdminToken) -> Result<(), RepoError> {
        todo!()
    }
}
