use diesel::Connection;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use std::path::Path;
use storage_diesel::{SqlitePool, create_pool};

pub(crate) const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub(crate) fn establish_test_connection(path: &Path) -> SqliteConnection {
    let db_url = path
        .to_str()
        .expect("test database path is not valid UTF-8");
    let mut connection =
        SqliteConnection::establish(db_url).expect("failed to connect to test database");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("failed to run test database migrations");
    connection
}

#[allow(dead_code)]
pub(crate) fn setup_test_db(path: &Path) -> SqlitePool {
    let db_url = path.to_str().unwrap();

    drop(establish_test_connection(path));

    create_pool(db_url, 16).expect("failed to create pool")
}
