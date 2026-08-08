mod mappings;
mod models;
mod schema;

use crate::mappings::MappingError;
use crate::models::{
    AdminTokenRow, NewAdminGrant, NewAdminToken, NewTransaction, NewUser, UserWithRoleRow,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use delta_core::ports::repo::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo};
use diesel::prelude::*;
use std::path::Path;
use thiserror::Error;

use delta_core::domain::{
    ActionRecord, AdminGrantId, Amount, PasswordHash, Transaction, TransactionId,
    TransactionSource, User, UserId,
};
use delta_core::services::auth::{AdminToken, TokenData, TokenKind};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use diesel::OptionalExtension;

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug, Error)]
pub enum PoolError {
    #[error("failed to create database pool")]
    Build(#[from] r2d2::Error),
}

pub const DEV_SQLITE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/dev.sqlite");

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
    async fn get_user(&self, key: &UserId) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        let key = key.to_owned();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .find(key.0.to_string())
                .first::<UserWithRoleRow>(conn)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn get_user_by_name(&self, user_name: &str) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        let user_name = user_name.to_owned();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .filter(username.eq(user_name))
                .first::<UserWithRoleRow>(conn)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn get_user_by_card(&self, user_card_number: u32) -> Result<User, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row = users_with_role
                .filter(card_number.eq(user_card_number as i64))
                .first::<UserWithRoleRow>(conn)?;

            Ok(User::try_from(&row)?)
        })
    }

    async fn list_users(&self) -> Result<Vec<User>, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let rows = users_with_role
                .order(id)
                .load::<UserWithRoleRow>(conn)?;
            rows.iter()
                .map(User::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(RepoError::from)
        })
    }

    async fn list_admins(&self) -> Result<Vec<User>, RepoError> {
        use crate::schema::users_with_role::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let rows = users_with_role
                .filter(role.eq("admin"))
                .order(id)
                .load::<UserWithRoleRow>(conn)?;
            rows.iter()
                .map(User::try_from)
                .collect::<Result<Vec<_>, _>>()
                .map_err(RepoError::from)
        })
    }

    async fn insert_user(&self, user: User, record: ActionRecord) -> Result<(), RepoError> {
        use crate::schema::users::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(users)
                .values(&NewUser {
                    id: user.id.0.to_string(),
                    name: user.name,
                    username: user.username,
                    program: user.program,
                    card_number: user.card_number as i64,
                    birthdate: user.birthdate.format("%Y-%m-%d").to_string(),
                    comments: user.comments,
                    balance: i64::from(user.balance),
                    spent: i64::from(user.spent),
                    created_at: record.at.timestamp(),
                    created_by: record.actor.0.to_string(),
                })
                .execute(conn)?;
            Ok(())
        })
    }

    async fn update_user(&self, user: User) -> Result<(), RepoError> {
        use crate::schema::users::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::update(users.find(user.id.0.to_string()))
                .set((
                    name.eq(user.name),
                    username.eq(user.username),
                    program.eq(user.program),
                    card_number.eq(user.card_number as i64),
                    comments.eq(user.comments),
                    balance.eq(i64::from(user.balance)),
                    spent.eq(i64::from(user.spent)),
                ))
                .execute(conn)?;
            Ok(())
        })
    }
}

#[async_trait]
impl AdminRepo for DieselRepo {
    async fn get_admin(&self, user_id_val: UserId) -> Result<PasswordHash, RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let hash = admins
                .filter(user_id.eq(user_id_val.0.to_string()))
                .filter(revoked_at.is_null())
                .select(password_hash)
                .first::<String>(conn)?;

            Ok(PasswordHash::parse(&hash)?)
        })
    }

    async fn grant_admin(
        &self,
        admin_grant_id: AdminGrantId,
        user_id_val: UserId,
        password_hash_val: PasswordHash,
        record: ActionRecord,
    ) -> Result<(), RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(admins)
                .values(&NewAdminGrant {
                    id: admin_grant_id.0.to_string(),
                    user_id: user_id_val.0.to_string(),
                    password_hash: password_hash_val.as_str().to_string(),
                    granted_at: record.at.timestamp(),
                    granted_by: record.actor.0.to_string(),
                })
                .execute(conn)?;
            Ok(())
        })
    }

    async fn revoke_admin(
        &self,
        user_id_val: UserId,
        record: ActionRecord,
    ) -> Result<(), RepoError> {
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
            .execute(conn)?;
            Ok(())
        })
    }

    async fn update_admin_password(
        &self,
        user_id_val: UserId,
        password_hash_val: PasswordHash,
    ) -> Result<(), RepoError> {
        use crate::schema::admins::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::update(admins.filter(user_id.eq(user_id_val.0.to_string())))
                .set(password_hash.eq(password_hash_val.as_str().to_string()))
                .execute(conn)?;
            Ok(())
        })
    }
}

#[async_trait]
impl TransactionRepo for DieselRepo {
    async fn spend(
        &self,
        tx_id: TransactionId,
        user_id_val: UserId,
        amount_val: Amount,
        dt: DateTime<Utc>,
    ) -> Result<Transaction, RepoError> {
        use crate::schema::transactions::dsl::transactions;
        use crate::schema::users::dsl::*;
        use crate::schema::users_with_role::dsl::users_with_role;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            conn.transaction::<_, RepoError, _>(|conn| {
                let user = {
                    // DB → row
                    let row = users_with_role
                        .find(user_id_val.0.to_string())
                        .first::<UserWithRoleRow>(conn)?;

                    // row → domain
                    let user = User::try_from(&row)?;

                    // business rule
                    user.deduct_balance(amount_val).map_err(RepoError::from)?
                };

                diesel::update(users.find(user_id_val.0.to_string()))
                    .set((
                        balance.eq::<i64>(user.balance.into()),
                        spent.eq::<i64>(user.spent.into()),
                    ))
                    .execute(conn)?;

                let tx = Transaction::Spend {
                    id: tx_id,
                    user_id: user_id_val,
                    amount: amount_val,
                    ts: dt,
                    source: TransactionSource::Live,
                };

                diesel::insert_into(transactions)
                    .values(&NewTransaction::from(&tx))
                    .execute(conn)?;
                Ok(tx)
            })
        })
    }

    async fn top_up(
        &self,
        tx_id: TransactionId,
        user_id_val: UserId,
        amount_val: Amount,
        approved_by_val: &UserId,
        dt: DateTime<Utc>,
    ) -> Result<Transaction, RepoError> {
        use crate::schema::transactions::dsl::transactions;
        use crate::schema::users::dsl::*;
        use crate::schema::users_with_role::dsl::users_with_role;

        let approved_by_id = *approved_by_val;

        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            conn.transaction::<_, RepoError, _>(|conn| {
                let user = {
                    // DB → row
                    let row = users_with_role
                        .find(user_id_val.0.to_string())
                        .first::<UserWithRoleRow>(conn)?;

                    // row → domain
                    let user = User::try_from(&row)?;

                    // business rule
                    user.add_balance(amount_val).map_err(RepoError::from)?
                };

                diesel::update(users.find(user_id_val.0.to_string()))
                    .set((
                        balance.eq::<i64>(user.balance.into()),
                        spent.eq::<i64>(user.spent.into()),
                    ))
                    .execute(conn)?;

                let tx = Transaction::TopUp {
                    id: tx_id,
                    user_id: user_id_val,
                    amount: amount_val,
                    ts: dt,
                    approved_by: approved_by_id,
                    source: TransactionSource::Live,
                };

                diesel::insert_into(transactions)
                    .values(&NewTransaction::from(&tx))
                    .execute(conn)?;
                Ok(tx)
            })
        })
    }
}

#[async_trait]
impl TokenRepo for DieselRepo {
    async fn insert_token(
        &self,
        token_arg: AdminToken,
        data: TokenData,
        created_at_arg: DateTime<Utc>,
    ) -> Result<(), RepoError> {
        use crate::schema::admin_tokens::dsl::*;
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::insert_into(admin_tokens)
                .values(&NewAdminToken {
                    token: token_arg.0.to_vec(),
                    user_id: data.user_id.0.to_string(),
                    expires_at: data.expires_at.timestamp(),
                    single_use: matches!(data.kind, TokenKind::SingleUse),
                    created_at: created_at_arg.timestamp(),
                })
                .execute(conn)?;
            Ok(())
        })
    }

    async fn get_token(
        &self,
        token: &AdminToken,
        dt: DateTime<Utc>,
    ) -> Result<Option<TokenData>, RepoError> {
        use crate::schema::admin_tokens::dsl::{
            admin_tokens, expired, expires_at as expires_at_col, token as token_col,
        };
        let token_vec = token.0.to_vec();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            let row_opt = admin_tokens
                .filter(token_col.eq(token_vec))
                .filter(expired.eq(false))
                .filter(expires_at_col.gt(dt.timestamp()))
                .first::<AdminTokenRow>(conn)
                .optional()?;

            let row = match row_opt {
                Some(row) => row,
                None => return Ok(None),
            };

            Ok(Some(TokenData {
                user_id: UserId::try_from(row.user_id.as_str()).map_err(|_| RepoError::Internal)?,
                expires_at: chrono::DateTime::from_timestamp(row.expires_at, 0)
                    .ok_or(RepoError::Internal)?,
                kind: if row.single_use {
                    TokenKind::SingleUse
                } else {
                    TokenKind::Session
                },
            }))
        })
    }

    async fn expire_token(&self, token: &AdminToken) -> Result<(), RepoError> {
        use crate::schema::admin_tokens::dsl::{admin_tokens, expired, token as token_col};
        let token_vec = token.0.to_vec();
        repo_call!(self.pool, |conn: &mut SqliteConnection| {
            diesel::update(admin_tokens.filter(token_col.eq(token_vec)))
                .set(
                    // For simplicity, we just set the expiration time to the current time
                    expired.eq(true),
                )
                .execute(conn)?;
            Ok(())
        })
    }
}
