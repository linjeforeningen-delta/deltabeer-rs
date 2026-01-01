use std::path::Path;
use std::process::{Command, Stdio};
use storage_diesel::{SqlitePool, create_pool};

pub fn setup_test_db(path: &Path) -> SqlitePool {
    let db_url = path.to_str().unwrap();

    // Run migrations via diesel CLI (simple + reliable)
    let status = Command::new("diesel")
        .env("DATABASE_URL", db_url)
        .arg("migration")
        .arg("run")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run migrations");

    if !status.success() {
        panic!("migrations failed");
    }

    create_pool(db_url).expect("failed to create pool")
}
