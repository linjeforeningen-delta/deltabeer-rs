mod common;

use common::TestEnv;
use delta_core::domain::{Amount, Role, User, UserId};
use delta_core::ports::repo::{AdminRepo, UserRepo};
use delta_core::services::auth::issue_admin_pass;
use delta_core::services::transactions::{spend, top_up};
use rand_core::RngCore;
use uuid::Uuid;

async fn setup_user(env: &common::TestEnv, name: &str, balance: u32) -> UserId {
    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: name.to_string(),
        username: name.to_lowercase(),
        card_number: rand_core::OsRng.next_u32(),
        role: Role::User,
        birthdate: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(balance),
        spent: Amount(0),
    };
    UserRepo::insert_user(
        &env.repo,
        user,
        delta_core::domain::ActionRecord {
            actor: user_id,
            at: env.clock.0,
        },
    )
    .await
    .unwrap();
    user_id
}

async fn setup_admin(env: &TestEnv, id: UserId, pass: &str) {
    let user = User {
        id,
        name: "Admin".to_string(),
        username: format!("admin-{}", Uuid::now_v7()),
        card_number: rand_core::OsRng.next_u32(),
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
            at: env.clock.0,
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
            at: env.clock.0,
        },
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_transaction_flow() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = setup_user(&env, "Bob", 100).await;

    spend(user_id, Amount(40), &ctx).await.unwrap();
    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(60));
    assert_eq!(user.spent, Amount(40));

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;
    let admin_token = issue_admin_pass(admin_id, "admin".to_string(), &ctx)
        .await
        .unwrap();

    top_up(user_id, Amount(50), admin_token, &ctx)
        .await
        .unwrap();
    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(110));
}

#[tokio::test]
async fn test_insufficient_funds() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = setup_user(&env, "PoorGuy", 10).await;

    let res = spend(user_id, Amount(20), &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_multiple_transactions_consistency() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = setup_user(&env, "Consistent", 1000).await;

    for _ in 0..10 {
        spend(user_id, Amount(10), &ctx).await.unwrap();
    }

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(900));
    assert_eq!(user.spent, Amount(100));

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    for _ in 0..5 {
        let admin_token = issue_admin_pass(admin_id, "admin".to_string(), &ctx)
            .await
            .unwrap();
        top_up(user_id, Amount(20), admin_token, &ctx)
            .await
            .unwrap();
    }

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(1000));
}

#[tokio::test]
async fn test_unauthorized_top_up() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = setup_user(&env, "Alice", 100).await;
    let fake_token = delta_core::services::auth::AdminToken([0u8; 32]);

    let res = top_up(user_id, Amount(50), fake_token, &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_revoked_admin_cannot_top_up() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    let user_id = setup_user(&env, "Alice", 100).await;

    let admin_token = issue_admin_pass(admin_id, "admin".to_string(), &ctx)
        .await
        .unwrap();
    assert!(top_up(user_id, Amount(10), admin_token, &ctx).await.is_ok());

    let root_id = UserId(Uuid::now_v7());
    setup_admin(&env, root_id, "root").await;
    let root_token = issue_admin_pass(root_id, "root".to_string(), &ctx)
        .await
        .unwrap();
    delta_core::services::auth::revoke_admin(root_token, admin_id, &ctx)
        .await
        .unwrap();

    let res = issue_admin_pass(admin_id, "admin".to_string(), &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_spend_exactly_zero() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = setup_user(&env, "Zero", 50).await;
    spend(user_id, Amount(50), &ctx).await.unwrap();

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(0));
}

#[tokio::test]
async fn test_top_up_non_existent_user() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;
    let admin_token = issue_admin_pass(admin_id, "admin".to_string(), &ctx)
        .await
        .unwrap();

    let res = top_up(UserId(Uuid::now_v7()), Amount(100), admin_token, &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_spend_non_existent_user() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let res = spend(UserId(Uuid::now_v7()), Amount(100), &ctx).await;
    assert!(res.is_err());
}
