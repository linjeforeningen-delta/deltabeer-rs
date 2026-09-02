use diesel::prelude::*;

mod common;

#[test]
fn migrations_apply_cleanly() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");

    let conn = common::establish_test_connection(&db_path);
    drop(conn);
}

#[test]
fn users_are_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
                       VALUES ('u1', 'Alice', 'alice', 123, '1990-01-01', 0, 0, 0, 'u1')",
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
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1'),
               ('a1', 'Admin', 'admin', 2, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by, revoked_at, revoked_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1', 100, 'a1')
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
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
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
fn transaction_source_is_restricted() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')",
    )
        .execute(&mut conn)
        .unwrap();

    let result = diesel::sql_query(
        "INSERT INTO transactions (id, user_id, kind, amount, source, approved_by, created_at)
         VALUES ('t1', 'u1', 'spend', 10, 'imported', NULL, 0)",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn approved_by_must_be_admin() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1'),
               ('u2', 'User2', 'user2', 2, '1990-01-01', 0, 0, 0, 'u1')
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
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1'),
               ('a1', 'Admin', 'admin', 2, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1')
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
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query("DELETE FROM users WHERE id = 'a1'").execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn admin_cannot_revoke_themselves() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result =
        diesel::sql_query("UPDATE admins SET revoked_at = 100, revoked_by = 'a1' WHERE id = 'g1'")
            .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn admin_revocation_must_be_complete() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1'),
               ('a2', 'Admin2', 'admin2', 2, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result =
        diesel::sql_query("UPDATE admins SET revoked_at = 100 WHERE id = 'g1'").execute(&mut conn);
    assert!(result.is_err());

    let result =
        diesel::sql_query("UPDATE admins SET revoked_by = 'a2' WHERE id = 'g1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn admin_revocation_is_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1'),
               ('a2', 'Admin2', 'admin2', 2, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by, revoked_at, revoked_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1', 100, 'a2')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    let result =
        diesel::sql_query("UPDATE admins SET revoked_at = 200 WHERE id = 'g1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn admin_token_identity_is_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admin_tokens (token, user_id, expires_at, single_use, created_at)
        VALUES ('t1', 'u1', 100, 1, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query("UPDATE admin_tokens SET token = 't2' WHERE token = 't1'")
        .execute(&mut conn);
    assert!(result.is_err());

    let result = diesel::sql_query("UPDATE admin_tokens SET user_id = 'u2' WHERE token = 't1'")
        .execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn admin_token_expiry_cannot_be_extended() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admin_tokens (token, user_id, expires_at, single_use, created_at)
        VALUES ('t1', 'u1', 100, 1, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    // Extending expiry should fail
    let result = diesel::sql_query("UPDATE admin_tokens SET expires_at = 200 WHERE token = 't1'")
        .execute(&mut conn);
    assert!(result.is_err());

    // Shortening expiry should succeed
    let result = diesel::sql_query("UPDATE admin_tokens SET expires_at = 50 WHERE token = 't1'")
        .execute(&mut conn);
    assert!(result.is_ok());
}

#[test]
fn user_fields_immutability() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
                       VALUES ('u1', 'Alice', 'alice', 123, '1990-01-01', 0, 0, 0, 'u1')",
    )
        .execute(&mut conn)
        .unwrap();

    let result = diesel::sql_query("UPDATE users SET birthdate = '1991-01-01' WHERE id = 'u1'")
        .execute(&mut conn);
    assert!(result.is_err());

    let result =
        diesel::sql_query("UPDATE users SET created_at = 100 WHERE id = 'u1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn transactions_are_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'spend', 10, NULL, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result =
        diesel::sql_query("UPDATE transactions SET amount = 20 WHERE id = 't1'").execute(&mut conn);
    assert!(result.is_err());

    let result = diesel::sql_query("DELETE FROM transactions WHERE id = 't1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn only_one_active_admin_per_user() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 0, 'a1')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    // Trying to add another active admin record for same user should fail due to unique index
    let result = diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g2', 'a1', 'hash2', 10, 'a1')
    ",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn user_basic_constraints() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    // Empty name
    let result = diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', '', 'user1', 1, '1990-01-01', 0, 0, 0, 'u1')",
    )
        .execute(&mut conn);
    assert!(result.is_err());

    // Empty username
    let result = diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', 'User', '', 1, '1990-01-01', 0, 0, 0, 'u1')",
    )
        .execute(&mut conn);
    assert!(result.is_err());

    // Card number out of range
    let result = diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', 'User', 'user1', 4294967296, '1990-01-01', 0, 0, 0, 'u1')",
    )
        .execute(&mut conn);
    assert!(result.is_err());

    // Negative balance
    let result = diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', 'User', 'user1', 1, '1990-01-01', -1, 0, 0, 'u1')",
    )
        .execute(&mut conn);
    assert!(result.is_err());

    // Negative spent
    let result = diesel::sql_query(
        "INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
         VALUES ('u1', 'User', 'user1', 1, '1990-01-01', 0, -1, 0, 'u1')",
    )
        .execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn admin_grant_records_are_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('a1', 'Admin', 'admin', 1, '1990-01-01', 0, 0, 0, 'a1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admins (id, user_id, password_hash, granted_at, granted_by)
        VALUES ('g1', 'a1', 'hash', 10, 'a1')
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result =
        diesel::sql_query("UPDATE admins SET granted_at = 20 WHERE id = 'g1'").execute(&mut conn);
    assert!(result.is_err());

    let result =
        diesel::sql_query("UPDATE admins SET granted_by = 'a2' WHERE id = 'g1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn transaction_amount_must_be_positive() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    let result = diesel::sql_query(
        "
        INSERT INTO transactions (id, user_id, kind, amount, approved_by, created_at)
        VALUES ('t1', 'u1', 'spend', -10, NULL, 0)
    ",
    )
    .execute(&mut conn);

    assert!(result.is_err());
}

#[test]
fn cannot_delete_admin_tokens() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admin_tokens (token, user_id, expires_at, single_use, created_at)
        VALUES ('t1', 'u1', 100, 1, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result =
        diesel::sql_query("DELETE FROM admin_tokens WHERE token = 't1'").execute(&mut conn);
    assert!(result.is_err());
}

#[test]
fn admin_token_created_at_is_immutable() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let mut conn = common::establish_test_connection(&db_path);

    diesel::sql_query(
        "
        INSERT INTO users (id, name, username, card_number, birthdate, balance, spent, created_at, created_by)
        VALUES ('u1', 'User', 'user', 1, '1990-01-01', 0, 0, 0, 'u1')
    ",
    )
        .execute(&mut conn)
        .unwrap();

    diesel::sql_query(
        "
        INSERT INTO admin_tokens (token, user_id, expires_at, single_use, created_at)
        VALUES ('t1', 'u1', 100, 1, 0)
    ",
    )
    .execute(&mut conn)
    .unwrap();

    let result = diesel::sql_query("UPDATE admin_tokens SET created_at = 10 WHERE token = 't1'")
        .execute(&mut conn);
    assert!(result.is_err());
}
