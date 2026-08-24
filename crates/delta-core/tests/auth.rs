mod common;

use chrono::{Duration, Utc};
use common::TestEnv;
use delta_core::domain::{Amount, AuthPolicy, Role, User, UserId};
use delta_core::ports::repo::{AdminRepo, UserRepo};
use delta_core::services::auth::{
    grant_admin, issue_admin_pass, issue_admin_session, login, update_password,
    validate_authorization,
};
use uuid::Uuid;

fn random_card_number() -> u32 {
    let mut bytes = [0u8; 4];
    getrandom::fill(&mut bytes).unwrap();
    u32::from_ne_bytes(bytes)
}

async fn setup_admin(env: &TestEnv, id: UserId, pass: &str) {
    let user = User {
        id,
        name: "Admin".to_string(),
        username: format!("admin-{}", Uuid::now_v7()),
        program: "Administration".to_string(),
        card_number: random_card_number(),
        role: Role::Admin,
        birthdate: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    };
    UserRepo::insert_user(
        &env.repo,
        user,
        delta_core::domain::ActionRecord {
            actor: id,
            at: Utc::now(),
        },
    )
    .await
    .unwrap();
    AdminRepo::grant_admin(
        &env.repo,
        delta_core::domain::AdminGrantId(Uuid::now_v7()),
        id,
        delta_core::domain::hash_password(pass),
        delta_core::domain::ActionRecord {
            actor: id,
            at: Utc::now(),
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_auth_flow() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let root_id = UserId(Uuid::now_v7());
    setup_admin(&env, root_id, "root").await;

    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: "Target".to_string(),
        username: "target".to_string(),
        program: "Computer Science".to_string(),
        card_number: 1,
        role: Role::User,
        birthdate: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    };
    UserRepo::insert_user(
        &env.repo,
        user,
        delta_core::domain::ActionRecord {
            actor: root_id,
            at: Utc::now(),
        },
    )
    .await
    .unwrap();

    let password = "secret-password".to_string();
    grant_admin(root_id, user_id, password.clone(), &ctx)
        .await
        .unwrap();

    let login_res = login(user_id, password, &ctx).await;
    assert!(login_res.is_ok());

    let login_fail = login(user_id, "wrong root_token".to_string(), &ctx).await;
    assert!(login_fail.is_err());
}

#[tokio::test]
async fn test_revoke_admin() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let root_id = UserId(Uuid::now_v7());
    setup_admin(&env, root_id, "root").await;

    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: "Target".to_string(),
        username: "target2".to_string(),
        program: "Computer Science".to_string(),
        card_number: 2,
        role: Role::User,
        birthdate: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    };
    UserRepo::insert_user(
        &env.repo,
        user,
        delta_core::domain::ActionRecord {
            actor: root_id,
            at: Utc::now(),
        },
    )
    .await
    .unwrap();

    grant_admin(root_id, user_id, "pass".to_string(), &ctx)
        .await
        .unwrap();

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.role, Role::Admin);

    delta_core::services::auth::revoke_admin(root_id, user_id, &ctx)
        .await
        .unwrap();

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.role, Role::User);

    let admin_check = AdminRepo::get_admin(&env.repo, user_id).await;
    assert!(admin_check.is_err());
}

#[tokio::test]
async fn test_session_token_flow() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin-pass").await;

    let session_token = issue_admin_session(admin_id, &AuthPolicy::default(), &ctx)
        .await
        .unwrap();

    let validate_session = validate_authorization(session_token.clone(), &ctx).await;
    assert_eq!(validate_session.unwrap(), admin_id);

    let validate_session_again = validate_authorization(session_token, &ctx).await;
    assert!(validate_session_again.is_ok());
}

#[tokio::test]
async fn test_token_expiration() {
    let mut env = TestEnv::new();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin-pass").await;

    let ctx = env.ctx();
    let pass_token = issue_admin_pass(
        admin_id,
        "admin-pass".to_string(),
        &AuthPolicy::default(),
        &ctx,
    )
    .await
    .unwrap();

    assert!(
        validate_authorization(pass_token.clone(), &ctx)
            .await
            .is_ok()
    );

    env.clock.0 = env.clock.0 + Duration::minutes(1);

    let ctx_new = env.ctx();
    let validate_expired = validate_authorization(pass_token, &ctx_new).await;
    assert!(validate_expired.is_err());
}

#[tokio::test]
async fn test_update_password_flow() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "old-pass").await;

    assert!(login(admin_id, "old-pass".to_string(), &ctx).await.is_ok());

    update_password(admin_id, "new-pass".to_string(), &ctx)
        .await
        .unwrap();

    assert!(login(admin_id, "old-pass".to_string(), &ctx).await.is_err());

    assert!(login(admin_id, "new-pass".to_string(), &ctx).await.is_ok());
}

// #[tokio::test]
// async fn test_unauthorized_grant_admin() {
//     let env = TestEnv::new();
//     let ctx = env.ctx();
//
//     let fake_token = AdminToken([0u8; 32]);
//     let target_id = UserId(Uuid::now_v7());
//
//     let res = grant_admin(fake_token, target_id, "password".to_string(), &ctx).await;
//     assert!(res.is_err());
// }

#[tokio::test]
async fn test_login_non_existent_user() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let res = login(UserId(Uuid::now_v7()), "pass".to_string(), &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_grant_admin_already_admin() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let root_id = UserId(Uuid::now_v7());
    setup_admin(&env, root_id, "root").await;

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    // Grant admin to someone who is already admin should work (idempotent/update password)
    let res = grant_admin(root_id, admin_id, "new-pass".to_string(), &ctx).await;
    assert!(res.is_ok());

    // Should be able to login with new pass
    assert!(login(admin_id, "new-pass".to_string(), &ctx).await.is_ok());
}

#[tokio::test]
async fn test_session_token_expiration() {
    let mut env = TestEnv::new();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin-pass").await;

    let ctx = env.ctx();

    let session_token = issue_admin_session(admin_id, &AuthPolicy::default(), &ctx)
        .await
        .unwrap();

    assert!(
        validate_authorization(session_token.clone(), &ctx)
            .await
            .is_ok()
    );

    // Advance clock by 4 minutes (session tokens last 3m)
    env.clock.0 = env.clock.0 + Duration::minutes(4);

    let ctx_new = env.ctx();
    let validate_expired = validate_authorization(session_token, &ctx_new).await;
    assert!(validate_expired.is_err());
}
