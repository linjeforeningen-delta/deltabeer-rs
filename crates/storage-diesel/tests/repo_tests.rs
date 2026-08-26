mod common;

use chrono::{Duration, NaiveDate, Utc};
use delta_core::{
    domain::{
        ActionRecord, AdminGrantId, Amount, Role, Transaction, TransactionId, User, UserId,
        hash_password,
    },
    ports::repo::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo},
    services::auth::{AdminToken, TokenData, TokenKind},
};
use storage_diesel::DieselRepo;
use tempfile::TempDir;
use uuid::Uuid;

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
        program: "Test Program".to_string(),
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
    let user_id = UserId(Uuid::now_v7());
    let user = create_test_user(user_id, "alice");
    let record = create_action_record(user_id);

    UserRepo::insert_user(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    let retrieved = UserRepo::get_user(&repo, &user_id)
        .await
        .expect("get failed");
    assert_eq!(retrieved.id, user.id);
    assert_eq!(retrieved.username, user.username);
}

#[tokio::test]
async fn test_user_repo_get_by_name_and_card() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let user = create_test_user(user_id, "bob");
    let record = create_action_record(user_id);

    UserRepo::insert_user(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    let by_name = UserRepo::get_user_by_name(&repo, "bob")
        .await
        .expect("get_by_name failed");
    assert_eq!(by_name.id, user.id);

    let by_card = UserRepo::get_user_by_card(&repo, user.card_number)
        .await
        .expect("get_by_card failed");
    assert_eq!(by_card.id, user.id);
}

#[tokio::test]
async fn test_user_repo_update() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let mut user = create_test_user(user_id, "charlie");
    let record = create_action_record(user_id);

    UserRepo::insert_user(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    user.name = "Charlie Updated".to_string();
    user.balance = Amount(100);
    UserRepo::update_user(&repo, user.clone())
        .await
        .expect("update failed");

    let retrieved = UserRepo::get_user(&repo, &user_id)
        .await
        .expect("get failed");
    assert_eq!(retrieved.name, "Charlie Updated");
    assert_eq!(retrieved.balance, Amount(100));
}

#[tokio::test]
async fn test_admin_repo_grant_and_get() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let admin_grant_id = AdminGrantId(Uuid::now_v7());
    let user = create_test_user(user_id, "admin_user");
    let record = create_action_record(user_id);

    UserRepo::insert_user(&repo, user, record)
        .await
        .expect("insert user failed");

    let password_hash = hash_password("hashed_password");
    AdminRepo::grant_admin(
        &repo,
        admin_grant_id,
        user_id,
        password_hash.clone(),
        record,
    )
    .await
    .expect("grant failed");

    let hash = AdminRepo::get_admin(&repo, user_id)
        .await
        .expect("get admin failed");
    assert_eq!(hash, password_hash);
}

#[tokio::test]
async fn test_admin_repo_revoke() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId(Uuid::now_v7());
    let admin_grant_id = AdminGrantId(Uuid::now_v7());
    let admin_user = create_test_user(admin_id, "admin");
    let other_id = UserId(Uuid::now_v7());
    let other_user = create_test_user(other_id, "other");

    UserRepo::insert_user(&repo, admin_user, create_action_record(admin_id))
        .await
        .unwrap();
    UserRepo::insert_user(&repo, other_user, create_action_record(other_id))
        .await
        .unwrap();

    AdminRepo::grant_admin(
        &repo,
        admin_grant_id,
        admin_id,
        hash_password("pass"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    // Revoke admin_id by other_id
    AdminRepo::revoke_admin(&repo, admin_id, create_action_record(other_id))
        .await
        .expect("revoke failed");

    let result = AdminRepo::get_admin(&repo, admin_id).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_transaction_repo_spend_atomic() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let tx_id = TransactionId(Uuid::now_v7());
    let mut user = create_test_user(user_id, "spender");
    user.balance = Amount(100);
    UserRepo::insert_user(&repo, user, create_action_record(user_id))
        .await
        .unwrap();

    let tx = TransactionRepo::spend(&repo, tx_id, user_id, Amount(40), Utc::now())
        .await
        .expect("spend failed");

    assert!(matches!(tx, Transaction::Spend { amount, .. } if amount == Amount(40)));

    let updated_user = UserRepo::get_user(&repo, &user_id).await.unwrap();
    assert_eq!(updated_user.balance, Amount(60));
    assert_eq!(updated_user.spent, Amount(40));
}

#[tokio::test]
async fn test_transaction_repo_topup_atomic() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let admin_id = UserId(Uuid::now_v7());
    let admin_grant_id = AdminGrantId(Uuid::now_v7());
    let tx_id = TransactionId(Uuid::now_v7());
    let user = create_test_user(user_id, "receiver");
    let admin_user = create_test_user(admin_id, "admin");
    let record = create_action_record(user_id);
    let admin_record = create_action_record(admin_id);

    UserRepo::insert_user(&repo, user, record).await.unwrap();
    UserRepo::insert_user(&repo, admin_user, admin_record)
        .await
        .unwrap();

    AdminRepo::grant_admin(
        &repo,
        admin_grant_id,
        admin_id,
        hash_password("hash"),
        admin_record,
    )
    .await
    .unwrap();

    let tx = TransactionRepo::top_up(&repo, tx_id, user_id, Amount(100), &admin_id, Utc::now())
        .await
        .expect("topup failed");

    assert!(
        matches!(tx, Transaction::TopUp { amount, approved_by, .. } if amount == Amount(100) && approved_by == admin_id)
    );

    let updated_user = UserRepo::get_user(&repo, &user_id).await.unwrap();
    assert_eq!(updated_user.balance, Amount(100));
}

#[tokio::test]
async fn test_token_repo_session() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    UserRepo::insert_user(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken([1; 32]);
    let data = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(1),
        kind: TokenKind::Session,
    };

    TokenRepo::insert_token(&repo, token.clone(), data, Utc::now())
        .await
        .expect("insert token failed");

    let retrieved = TokenRepo::get_token(&repo, &token, Utc::now())
        .await
        .expect("get token failed")
        .expect("token not found");
    assert_eq!(retrieved.user_id, user_id);
    assert_eq!(retrieved.kind, TokenKind::Session);
}

#[tokio::test]
async fn test_token_repo_single_use() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    UserRepo::insert_user(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken([2; 32]);
    let data = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::minutes(5),
        kind: TokenKind::SingleUse,
    };

    TokenRepo::insert_token(&repo, token.clone(), data, Utc::now())
        .await
        .expect("insert token failed");

    let retrieved = TokenRepo::get_token(&repo, &token, Utc::now())
        .await
        .expect("get token failed")
        .expect("token not found");
    assert_eq!(retrieved.kind, TokenKind::SingleUse);
}

#[tokio::test]
async fn test_user_repo_conflict() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let user = create_test_user(user_id, "alice");
    let record = create_action_record(user_id);

    UserRepo::insert_user(&repo, user.clone(), record)
        .await
        .expect("insert failed");

    // Duplicate insert
    let result = UserRepo::insert_user(&repo, user, record).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_not_found() {
    let (repo, _dir) = setup().await;
    let result = UserRepo::get_user(&repo, &UserId(Uuid::now_v7())).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_admin_repo_get_not_found() {
    let (repo, _dir) = setup().await;
    let result = AdminRepo::get_admin(&repo, UserId(Uuid::now_v7())).await;
    assert!(matches!(result, Err(RepoError::NotFound)));
}

#[tokio::test]
async fn test_token_repo_get_not_found() {
    let (repo, _dir) = setup().await;
    let result = TokenRepo::get_token(&repo, &AdminToken([3; 32]), Utc::now()).await;
    assert!(matches!(result, Ok(None)));
}

#[tokio::test]
async fn test_user_repo_insert_duplicate_username() {
    let (repo, _dir) = setup().await;
    let user1_id = UserId(Uuid::now_v7());
    let user1 = create_test_user(user1_id, "alice");
    UserRepo::insert_user(&repo, user1, create_action_record(user1_id))
        .await
        .unwrap();

    let user2_id = UserId(Uuid::now_v7());
    let mut user2 = create_test_user(user2_id, "alice"); // same username
    user2.card_number = 9999; // different card number
    let result = UserRepo::insert_user(&repo, user2, create_action_record(user1_id)).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_insert_duplicate_card_number() {
    let (repo, _dir) = setup().await;
    let user1_id = UserId(Uuid::now_v7());
    let user1 = create_test_user(user1_id, "alice");
    UserRepo::insert_user(&repo, user1.clone(), create_action_record(user1_id))
        .await
        .unwrap();

    let user2_id = UserId(Uuid::now_v7());
    let mut user2 = create_test_user(user2_id, "bob");
    user2.card_number = user1.card_number; // same card number
    let result = UserRepo::insert_user(&repo, user2, create_action_record(user1_id)).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_user_repo_update_immutable_fields_fail() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let user = create_test_user(user_id, "alice");
    UserRepo::insert_user(&repo, user.clone(), create_action_record(user_id))
        .await
        .unwrap();

    // up.sql: CREATE TRIGGER prevent_users_mutation BEFORE UPDATE OF id, birthdate, created_at, created_by ON users
    // DieselRepo::update does NOT set id, birthdate, created_at, or created_by.
    // So the trigger is NOT fired even if we change them in the User struct, because they are not in the .set() call.

    // Let's verify that even if we change birthdate in the struct, it's NOT updated in DB.
    let mut updated_user = user.clone();
    updated_user.birthdate = NaiveDate::from_ymd_opt(2000, 2, 2).unwrap();
    updated_user.name = "Alice Updated".to_string();

    UserRepo::update_user(&repo, updated_user)
        .await
        .expect("update failed");

    let retrieved = UserRepo::get_user(&repo, &user_id).await.unwrap();
    assert_eq!(retrieved.name, "Alice Updated");
    assert_eq!(retrieved.birthdate, user.birthdate); // Should remain unchanged
}

#[tokio::test]
async fn test_token_repo_overwrite() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    UserRepo::insert_user(
        &repo,
        create_test_user(user_id, "user"),
        create_action_record(user_id),
    )
    .await
    .unwrap();

    let token = AdminToken([4; 32]);
    let data1 = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(1),
        kind: TokenKind::Session,
    };
    TokenRepo::insert_token(&repo, token.clone(), data1, Utc::now())
        .await
        .unwrap();

    let data2 = TokenData {
        user_id,
        expires_at: Utc::now() + Duration::hours(2),
        kind: TokenKind::SingleUse,
    };
    // Re-inserting same token should conflict
    let result = TokenRepo::insert_token(&repo, token, data2, Utc::now()).await;
    assert!(matches!(result, Err(RepoError::Conflict)));
}

#[tokio::test]
async fn test_admin_repo_revoke_already_revoked() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId(Uuid::now_v7());
    let admin_grant_id = AdminGrantId(Uuid::now_v7());
    let other_id = UserId(Uuid::now_v7());
    UserRepo::insert_user(
        &repo,
        create_test_user(admin_id, "admin"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();
    UserRepo::insert_user(
        &repo,
        create_test_user(other_id, "other"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    AdminRepo::grant_admin(
        &repo,
        admin_grant_id,
        admin_id,
        hash_password("pass"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    AdminRepo::revoke_admin(&repo, admin_id, create_action_record(other_id))
        .await
        .unwrap();

    // Second revocation
    // DieselRepo::revoke updates where revoked_at IS NULL.
    // If already revoked, no row matches, execute(conn) returns 0.
    // map_diesel_error might not return error for 0 rows updated unless specified.
    let result = AdminRepo::revoke_admin(&repo, admin_id, create_action_record(other_id)).await;
    // Actually, SQL trigger prevent_admin_revocation_rules Rule 0: OLD.revoked_at IS NOT NULL RAISE ABORT.
    // But the WHERE clause in DieselRepo::revoke prevents it from matching already revoked row.
    // So it should just do nothing and return Ok(()).
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_admin_repo_self_revocation_forbidden() {
    let (repo, _dir) = setup().await;
    let admin_id = UserId(Uuid::now_v7());
    let admin_grant_id = AdminGrantId(Uuid::now_v7());
    UserRepo::insert_user(
        &repo,
        create_test_user(admin_id, "admin"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();
    AdminRepo::grant_admin(
        &repo,
        admin_grant_id,
        admin_id,
        hash_password("hash"),
        create_action_record(admin_id),
    )
    .await
    .unwrap();

    // Self revocation
    let result = AdminRepo::revoke_admin(&repo, admin_id, create_action_record(admin_id)).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_repo_update_mutable_fields() {
    let (repo, _dir) = setup().await;
    let user_id = UserId(Uuid::now_v7());
    let mut user = create_test_user(user_id, "alice");
    UserRepo::insert_user(&repo, user.clone(), create_action_record(user_id))
        .await
        .unwrap();

    user.name = "Alice Updated".to_string();
    user.balance = Amount(1000);
    user.comments = "Updated comments".to_string();

    UserRepo::update_user(&repo, user.clone())
        .await
        .expect("update failed");

    let retrieved = UserRepo::get_user(&repo, &user_id).await.unwrap();
    assert_eq!(retrieved.name, "Alice Updated");
    assert_eq!(retrieved.balance, Amount(1000));
    assert_eq!(retrieved.comments, "Updated comments");
}
