mod common;

use chrono::{Duration, NaiveDate, Utc};
use delta_core::domain::{ActionRecord, Amount, Role, Transaction, TransactionId, User, UserId};
use delta_core::ports::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo};
use delta_core::services::auth::{AdminToken, TokenData, TokenKind};
use storage_diesel::DieselRepo;
use tempfile::TempDir;

async fn setup() -> (DieselRepo, TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("test.sqlite");
    let pool = common::setup_test_db(&db_path);
    (DieselRepo::new(pool), dir)
}

fn create_test_user(id: UserId, username: &str) -> User {
    let mut card_number = 0u32;
    for (i, b) in username.as_bytes().iter().enumerate() {
        card_number = card_number.wrapping_add((*b as u32) << (i % 4));
    }
    User {
        id,
        name: format!("Name {}", username),
        username: username.to_string(),
        card_number,
        role: Role::User,
        birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "Test user".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    }
}

fn create_action_record(actor: UserId) -> ActionRecord {
    ActionRecord {
        actor,
        at: Utc::now(),
    }
}

#[tokio::test]
async fn test_user_repo_insert_and_get() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "alice");
    let record = create_action_record(user_id);

    UserRepo::insert(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    let retrieved = UserRepo::get(&repo, &user_id).await.expect("get failed");
    assert_eq!(retrieved.id, user.id);
    assert_eq!(retrieved.username, user.username);
}

#[tokio::test]
async fn test_user_repo_get_by_name_and_card() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "bob");
    let record = create_action_record(user_id);

    UserRepo::insert(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    let by_name = UserRepo::get_by_name(&repo, "bob")
        .await
        .expect("get_by_name failed");
    assert_eq!(by_name.id, user.id);

    let by_card = UserRepo::get_by_card(&repo, user.card_number)
        .await
        .expect("get_by_card failed");
    assert_eq!(by_card.id, user.id);
}

#[tokio::test]
async fn test_user_repo_update() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let mut user = create_test_user(user_id, "charlie");
    let record = create_action_record(user_id);

    UserRepo::insert(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    user.name = "Charlie Updated".to_string();
    user.balance = Amount(100);
    UserRepo::update(&repo, user.clone())
        .await
        .expect("update failed");

    let retrieved = UserRepo::get(&repo, &user_id).await.expect("get failed");
    assert_eq!(retrieved.name, "Charlie Updated");
    assert_eq!(retrieved.balance, Amount(100));
}

#[tokio::test]
async fn test_admin_repo_grant_and_get() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "admin_user");
    let record = create_action_record(user_id);

    UserRepo::insert(&repo, user, record)
        .await
        .expect("insert user failed");

    AdminRepo::grant(&repo, user_id, "hashed_password".to_string(), record)
        .await
        .expect("grant failed");

    let hash = AdminRepo::get(&repo, user_id)
        .await
        .expect("get admin failed");
    assert_eq!(hash, "hashed_password");
}

#[tokio::test]
async fn test_admin_repo_revoke() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId::new();
    let admin_user = create_test_user(admin_id, "admin");
    let other_id = UserId::new();
    let other_user = create_test_user(other_id, "other");

    UserRepo::insert(&repo, admin_user, create_action_record(admin_id))
        .await
        .unwrap();
    UserRepo::insert(&repo, other_user, create_action_record(other_id))
        .await
        .unwrap();

    AdminRepo::grant(
        &repo,
        admin_id,
        "pass".to_string(),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    // Revoke admin_id by other_id
    AdminRepo::revoke(&repo, admin_id, create_action_record(other_id))
        .await
        .expect("revoke failed");

    let result = AdminRepo::get(&repo, admin_id).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_transaction_repo_insert_spend() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "spender");
    UserRepo::insert(&repo, user, create_action_record(user_id))
        .await
        .unwrap();

    let tx = Transaction::Spend {
        id: TransactionId::new(),
        user_id,
        amount: Amount(50),
        ts: Utc::now(),
    };

    TransactionRepo::insert(&repo, tx)
        .await
        .expect("insert spend failed");
}

#[tokio::test]
async fn test_transaction_repo_insert_topup() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let admin_id = UserId::new();

    UserRepo::insert(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();
    UserRepo::insert(
        &repo,
        create_test_user(admin_id, "admin"),
        create_action_record(user_id),
    )
    .await
    .unwrap();
    AdminRepo::grant(
        &repo,
        admin_id,
        "pass".to_string(),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    let tx = Transaction::TopUp {
        id: TransactionId::new(),
        user_id,
        amount: Amount(100),
        ts: Utc::now(),
        approved_by: admin_id,
    };

    TransactionRepo::insert(&repo, tx)
        .await
        .expect("insert topup failed");
}

#[tokio::test]
async fn test_token_repo_session() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    UserRepo::insert(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken("session_token".to_string());
    let data = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(1),
        kind: TokenKind::Session,
    };

    TokenRepo::insert(&repo, token.clone(), data, Utc::now())
        .await
        .expect("insert token failed");

    let retrieved = TokenRepo::get(&repo, &token)
        .await
        .expect("get token failed");
    assert_eq!(retrieved.user_id, user_id);
    assert_eq!(retrieved.kind, TokenKind::Session);
}

#[tokio::test]
async fn test_token_repo_single_use() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    UserRepo::insert(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken("single_use_token".to_string());
    let data = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::minutes(5),
        kind: TokenKind::SingleUse,
    };

    TokenRepo::insert(&repo, token.clone(), data, Utc::now())
        .await
        .expect("insert token failed");

    let retrieved = TokenRepo::get(&repo, &token)
        .await
        .expect("get token failed");
    assert_eq!(retrieved.kind, TokenKind::SingleUse);
}

#[tokio::test]
async fn test_user_repo_conflict() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "alice");
    let record = create_action_record(user_id);

    UserRepo::insert(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    // Duplicate insert
    let result = UserRepo::insert(&repo, user, record).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_not_found() {
    let (repo, _dir) = setup().await;
    let result = UserRepo::get(&repo, &UserId::new()).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_admin_repo_get_not_found() {
    let (repo, _dir) = setup().await;
    let result = AdminRepo::get(&repo, UserId::new()).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_token_repo_get_not_found() {
    let (repo, _dir) = setup().await;
    let result = TokenRepo::get(&repo, &AdminToken("non_existent".to_string())).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_user_repo_insert_duplicate_username() {
    let (repo, _dir) = setup().await;
    let user1_id = UserId::new();
    let user1 = create_test_user(user1_id, "alice");
    UserRepo::insert(&repo, user1, create_action_record(user1_id))
        .await
        .unwrap();

    let user2_id = UserId::new();
    let mut user2 = create_test_user(user2_id, "alice"); // same username
    user2.card_number = 9999; // different card number
    let result = UserRepo::insert(&repo, user2, create_action_record(user1_id)).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_insert_duplicate_card_number() {
    let (repo, _dir) = setup().await;
    let user1_id = UserId::new();
    let user1 = create_test_user(user1_id, "alice");
    UserRepo::insert(&repo, user1.clone(), create_action_record(user1_id))
        .await
        .unwrap();

    let user2_id = UserId::new();
    let mut user2 = create_test_user(user2_id, "bob");
    user2.card_number = user1.card_number; // same card number
    let result = UserRepo::insert(&repo, user2, create_action_record(user1_id)).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_update_immutable_fields_fail() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let user = create_test_user(user_id, "alice");
    UserRepo::insert(&repo, user.clone(), create_action_record(user_id))
        .await
        .unwrap();

    // up.sql: CREATE TRIGGER prevent_users_mutation BEFORE UPDATE OF id, birthdate, created_at, created_by ON users
    // DieselRepo::update does NOT set id, birthdate, created_at, or created_by.
    // So the trigger is NOT fired even if we change them in the User struct, because they are not in the .set() call.

    // Let's verify that even if we change birthdate in the struct, it's NOT updated in DB.
    let mut updated_user = user.clone();
    updated_user.birthdate = NaiveDate::from_ymd_opt(2000, 2, 2).unwrap();
    updated_user.name = "Alice Updated".to_string();

    UserRepo::update(&repo, updated_user)
        .await
        .expect("update failed");

    let retrieved = UserRepo::get(&repo, &user_id).await.unwrap();
    assert_eq!(retrieved.name, "Alice Updated");
    assert_eq!(retrieved.birthdate, user.birthdate); // Should remain unchanged
}

#[tokio::test]
async fn test_token_repo_overwrite() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    UserRepo::insert(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken("token".to_string());
    let data1 = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(1),
        kind: TokenKind::Session,
    };
    TokenRepo::insert(&repo, token.clone(), data1, Utc::now())
        .await
        .unwrap();

    let data2 = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(2),
        kind: TokenKind::SingleUse,
    };
    // Re-inserting same token should conflict
    let result = TokenRepo::insert(&repo, token, data2, Utc::now()).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_admin_repo_revoke_already_revoked() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId::new();
    let other_id = UserId::new();
    UserRepo::insert(
        &repo,
        create_test_user(admin_id, "admin"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();
    UserRepo::insert(
        &repo,
        create_test_user(other_id, "other"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    AdminRepo::grant(
        &repo,
        admin_id,
        "hash".to_string(),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    AdminRepo::revoke(&repo, admin_id, create_action_record(other_id))
        .await
        .unwrap();

    // Second revocation
    // DieselRepo::revoke updates where revoked_at IS NULL.
    // If already revoked, no row matches, execute(conn) returns 0.
    // map_diesel_error might not return error for 0 rows updated unless specified.
    let result = AdminRepo::revoke(&repo, admin_id, create_action_record(other_id)).await;
    // Actually, SQL trigger prevent_admin_revocation_rules Rule 0: OLD.revoked_at IS NOT NULL RAISE ABORT.
    // But the WHERE clause in DieselRepo::revoke prevents it from matching already revoked row.
    // So it should just do nothing and return Ok(()).
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_admin_repo_self_revocation_forbidden() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId::new();
    UserRepo::insert(
        &repo,
        create_test_user(admin_id, "admin"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();
    AdminRepo::grant(
        &repo,
        admin_id,
        "hash".to_string(),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    // Self revocation
    let result = AdminRepo::revoke(&repo, admin_id, create_action_record(admin_id)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_repo_update_mutable_fields() {
    let (repo, _dir) = setup().await;
    let user_id = UserId::new();
    let mut user = create_test_user(user_id, "alice");
    UserRepo::insert(&repo, user.clone(), create_action_record(user_id))
        .await
        .unwrap();

    user.name = "Alice Updated".to_string();
    user.balance = Amount(1000);
    user.comments = "Updated comments".to_string();

    UserRepo::update(&repo, user.clone())
        .await
        .expect("update failed");

    let retrieved = UserRepo::get(&repo, &user_id).await.unwrap();
    assert_eq!(retrieved.name, "Alice Updated");
    assert_eq!(retrieved.balance, Amount(1000));
    assert_eq!(retrieved.comments, "Updated comments");
}
