use async_trait::async_trait;
use delta_core::{
    domain::*,
    ports::{RepoError, UserRepo},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, params, OpenFlags};
use tokio::task;

mod mappings;

fn make_pool(path: &str) -> Result<Pool<SqliteConnectionManager>, RepoError> {
    let flags = OpenFlags::SQLITE_OPEN_READ_WRITE
        | OpenFlags::SQLITE_OPEN_CREATE
        | OpenFlags::SQLITE_OPEN_FULL_MUTEX; // important for thread-safety

    let manager = SqliteConnectionManager::file(path).with_flags(flags);
    let pool = Pool::new(manager)?;

    // Initialize the DB once (WAL plus foreign keys)
    {
        let conn = pool.get()?;
        // ORDER: WAL mode first, then foreign keys (and any other pragmas you want)
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    }

    Ok(pool)
}

#[derive(Clone)]
pub struct SqliteUserRepo {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteUserRepo {
    pub fn new(path: &str) -> Result<Self, RepoError> {
        let pool = make_pool(path)?;
        Ok(Self { pool })
    }
}

pub enum UserKey<'a> {
    Id(UserId),
    Username(&'a str),
    Card(&'a str),
}

#[async_trait]
impl UserRepo for SqliteUserRepo {
    async fn get(&self, key: &str) -> Result<User, RepoError> {
        let pool = self.pool.clone();
        let raw = key.to_owned();

        tokio::task::spawn_blocking(move || -> Result<User, RepoError> {
            let conn = pool.get()?;
            conn.execute_batch("PRAGMA foreign_keys=ON;")?;



            let mut stmt = conn.prepare("
                SELECT id, username, name, card_number, role, birthdate, comments, balance, spent
                FROM users
                WHERE id = ?1 OR username = ?1 OR card_number = ?1
                LIMIT 1;
            ")?;
            let user = stmt.query_row(params![raw], mappings::row_to_user).map_err(|e| {
                if matches!(e, rusqlite::Error::QueryReturnedNoRows) {
                    RepoError::NotFound
                } else {
                    e.into()
                }
            })?;

            Ok(user)
        })
            .await?
    }

    async fn create(&self, user: User) -> Result<(), RepoError> {
        let pool = self.pool.clone();

        tokio::task::spawn_blocking(move || -> Result<(), RepoError> {
            let conn = pool.get()?;
            conn.execute_batch("PRAGMA foreign_keys=ON;")?;

            conn.execute(
                r#"
                INSERT INTO users (
                    id, username, name, card_nuPRAGMA foreign_keysmber, password, role, birthdate, comments, balance, spent
                )
                VALUES (
                    :id, :username, :name, :card_number, :password, :role, :birthdate, :comments, :balance, :spent
                )
                ON CONFLICT DO NOTHING;
                "#,
                named_params! {
                    ":id": user.id.0,
                    ":username": user.username,
                    ":name": user.name,
                    ":card_number": user.card_number,
                    ":role": user.role,
                    ":birthdate": user.birthdate,
                    ":comments": user.comments,
                    ":balance": user.balance,
                    ":spent": user.spent,
                },
            )?;


            Ok(())
            }).await?
    }

    async fn update(&self, user: User) -> Result<(), RepoError> {
        let pool = self.pool.clone();
    }
}
