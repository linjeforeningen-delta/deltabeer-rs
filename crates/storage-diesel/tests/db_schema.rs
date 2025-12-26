use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path::Path;
use std::process::Command;

pub fn setup_test_db(path: &Path) -> SqliteConnection {
    let db_url = path.to_str().unwrap();

    // Run migrations via diesel CLI (simple + reliable)
    Command::new("diesel")
        .env("DATABASE_URL", db_url)
        .arg("migration")
        .arg("run")
        .status()
        .expect("failed to run migrations");

    SqliteConnection::establish(db_url).expect("failed to connect to test db")
}

#[test]
fn migrations_apply_cleanly() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");

    let conn = setup_test_db(&db_path);
    drop(conn);
}

#[test]
fn users_are_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, role, birthdate)
                       VALUES ('u1', 'Alice', 'alice', 123, 'user', '1990-01-01')",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query("DELETE FROM users WHERE id = 'u1'").execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn topup_requires_active_admin() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, role, birthdate)
        VALUES ('u1', 'User', 'user', 1, 'user', '1990-01-01'),
               ('a1', 'Admin', 'admin', 2, 'admin', '1990-01-01')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (user_id, password_hash, created_at, active)
        VALUES ('a1', 'hash', 0, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'topup', 100, 'a1', 0)
    ",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn spend_cannot_have_approved_by() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, role, birthdate)
        VALUES ('u1', 'User', 'user', 1, 'user', '1990-01-01')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'spend', 10, 'u1', 0)
    ",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn approved_by_must_be_admin() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, role, birthdate)
        VALUES ('u1', 'User', 'user', 1, 'user', '1990-01-01'),
               ('u2', 'User2', 'user2', 2, 'user', '1990-01-01')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'topup', 100, 'u2', 0)
    ",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn active_admin_can_approve_topup() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, role, birthdate)
        VALUES ('u1', 'User', 'user', 1, 'user', '1990-01-01'),
               ('a1', 'Admin', 'admin', 2, 'admin', '1990-01-01')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (user_id, password_hash, created_at, active)
        VALUES ('a1', 'hash', 0, 1)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'topup', 100, 'a1', 0)
    ",
    )
    .execute(&mut conn);

    assert!(result.is_ok());
}

#[test]
fn cannot_delete_admin_user() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = setup_test_db(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, role, birthdate)
        VALUES ('a1', 'Admin', 'admin', 1, 'admin', '1990-01-01')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (user_id, password_hash, created_at, active)
        VALUES ('a1', 'hash', 0, 1)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query("DELETE FROM users WHERE id = 'a1'").execute(&mut conn);

    assert!(result.is_err());
}
