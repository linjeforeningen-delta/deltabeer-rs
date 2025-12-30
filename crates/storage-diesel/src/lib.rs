mod mappings;
mod models;
mod schema;

use crate::mappings::MappingError;
use crate::models::{
    AdminTokenRow, NewAdminGrant, NewAdminToken, NewTransaction, NewUser, UserWithRoleRow,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use delta_core::ports::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo};
use diesel::prelude::*;
use thiserror::Error;

use delta_core::domain::{ActionRecord, Transaction, User, UserId};
use delta_core::services::auth::{AdminToken, TokenData, TokenKind};
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

fn map_diesel_error(e: diesel::result::Error) -> RepoError {
    match e {
        diesel::result::Error::NotFound => {
            tracing::info!("entity not found");
            RepoError::NotFound
        }
        diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        ) => {
            tracing::warn!("conflict with existing data");
            RepoError::Conflict
        }
        _ => {
            tracing::error!(error = ?e, "storage layer failure");
            RepoError::StorageFailure
        }
    }
}

impl From<MappingError> for RepoError {
    fn from(e: MappingError) -> Self {
        tracing::error!(error = ?e, "data mapping failure");
        RepoError::Internal
    }
}

macro_rules! repo_call {
    ($pool:expr, $block:expr) => {{
        let pool = $pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(|e| {
                tracing::error!(error = ?e, "failed to get connection from pool");
                RepoError::StorageFailure
            })?;
            $block(&mut conn)
        })
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, "blocking task failed");
            RepoError::Internal
        })?
    }};
}

#[async_trait]
impl UserRepo for DieselRepo {
    async fn get(&self, key: &UserId) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        let key = key.to_owned();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .find(key.0.to_string())
                .first::<UserWithRoleRow>(conn)
                .map_err(map_diesel_error)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn get_by_name(&self, user_name: &str) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        let user_name = user_name.to_owned();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .filter(username.eq(user_name))
                .first::<UserWithRoleRow>(conn)
                .map_err(map_diesel_error)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn get_by_card(&self, user_card_number: u32) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .filter(card_number.eq(user_card_number as i64))
                .first::<UserWithRoleRow>(conn)
                .map_err(map_diesel_error)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn insert(&self, user: User, record: ActionRecord) -> Result<(), RepoError> {
        use crate::schema::users::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(users)
                .values(&NewUser {
                    id: user.id.0.to_string(),
                    name: user.name,
                    username: user.username,
                    card_number: user.card_number as i64,
                    birthdate: user.birthdate.format("%Y-%m-%d").to_string(),
                    comments: user.comments,
                    balance: i64::from(user.balance),
                    spent: i64::from(user.spent),
                    created_at: record.at.timestamp(),
                    created_by: record.actor.0.to_string(),
                })
                .execute(conn)
                .map_err(map_diesel_error)?;
            Ok(())
        })
    }

    async fn update(&self, user: User) -> Result<(), RepoError> {
        use crate::schema::users::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::update(users.find(user.id.0.to_string()))
                .set((
                    name.eq(user.name),
                    username.eq(user.username),
                    card_number.eq(user.card_number as i64),
                    comments.eq(user.comments),
                    balance.eq(i64::from(user.balance)),
                    spent.eq(i64::from(user.spent)),
                ))
                .execute(conn)
                .map_err(map_diesel_error)?;
            Ok(())
        })
    }
}

#[async_trait]
impl AdminRepo for DieselRepo {
    async fn get(&self, user_id_val: UserId) -> Result<String, RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let hash = admins
                .filter(user_id.eq(user_id_val.0.to_string()))
                .filter(revoked_at.is_null())
                .select(password_hash)
                .first::<String>(conn)
                .map_err(map_diesel_error)?;

            Ok(hash)
        })
    }

    async fn grant(
        &self,
        user_id_val: UserId,
        data: String,
        record: ActionRecord,
    ) -> Result<(), RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(admins)
                .values(&NewAdminGrant {
                    id: uuid::Uuid::new_v4().to_string(),
                    user_id: user_id_val.0.to_string(),
                    password_hash: data,
                    granted_at: record.at.timestamp(),
                    granted_by: record.actor.0.to_string(),
                })
                .execute(conn)
                .map_err(map_diesel_error)?;
            Ok(())
        })
    }

    async fn revoke(&self, user_id_val: UserId, record: ActionRecord) -> Result<(), RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::update(
                admins
                    .filter(user_id.eq(user_id_val.0.to_string()))
                    .filter(revoked_at.is_null()),
            )
            .set((
                revoked_at.eq(record.at.timestamp()),
                revoked_by.eq(record.actor.0.to_string()),
            ))
            .execute(conn)
            .map_err(map_diesel_error)?;
            Ok(())
        })
    }
}

#[async_trait]
impl TransactionRepo for DieselRepo {
    async fn insert(&self, tx: Transaction) -> Result<(), RepoError> {
        use crate::schema::transactions::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(transactions)
                .values(&NewTransaction::from(tx))
                .execute(conn)
                .map_err(map_diesel_error)?;
            Ok(())
        })
    }
}

#[async_trait]
impl TokenRepo for DieselRepo {
    async fn insert(
        &self,
        token_arg: AdminToken,
        data: TokenData,
        created_at_arg: DateTime<Utc>,
    ) -> Result<(), RepoError> {
        use crate::schema::admin_tokens::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(admin_tokens)
                .values(&NewAdminToken {
                    token: token_arg.0,
                    user_id: data.user_id.0.to_string(),
                    expires_at: data.expires_at.timestamp(),
                    single_use: matches!(data.kind, TokenKind::SingleUse),
                    created_at: created_at_arg.timestamp(),
                })
                .execute(conn)
                .map_err(map_diesel_error)?;
            Ok(())
        })
    }

    async fn get(&self, token: &AdminToken) -> Result<TokenData, RepoError> {
        use crate::schema::admin_tokens::dsl::{admin_tokens, token as token_col};
        let token_val = token.0.clone();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = admin_tokens
                .filter(token_col.eq(token_val))
                .first::<AdminTokenRow>(conn)
                .map_err(map_diesel_error)?;

            Ok(TokenData {
                user_id: UserId::try_from(row.user_id.as_str()).map_err(|_| RepoError::Internal)?,
                expires_at: chrono::DateTime::from_timestamp(row.expires_at, 0)
                    .ok_or(RepoError::Internal)?,
                kind: if row.single_use {
                    TokenKind::SingleUse
                } else {
                    TokenKind::Session
                },
            })
        })
    }
}
