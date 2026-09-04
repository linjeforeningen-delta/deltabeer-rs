mod common;

use common::TestEnv;
use delta_core::{
    domain::{Amount, AuthPolicy, Role, User, UserId},
    ports::repo::{AdminRepo, UserRepo},
    services::{
        auth::issue_admin_pass,
        transactions::{spend, top_up},
    },
};
use uuid::Uuid;

fn random_card_number() -> u32 {
    let mut bytes = [0u8; 4];
    getrandom::fill(&mut bytes).unwrap();
    u32::from_ne_bytes(bytes)
}

async fn setup_user(env: &common::TestEnv, name: &str, balance: u32) -> UserId {
    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: name.to_string(),
        username: name.to_lowercase(),
        program: "Computer Science".to_string(),
        card_number: random_card_number(),
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

async fn setup_admin(env: &TestEnv, id: UserId) -> String {
    let password = common::test_password();
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
            at: env.clock.0,
        },
    )
    .await
    .unwrap();
    AdminRepo::grant_admin(
        &env.repo,
        delta_core::domain::AdminGrantId(Uuid::now_v7()),
        id,
        delta_core::domain::hash_password(&password),
        delta_core::domain::ActionRecord {
            actor: id,
            at: env.clock.0,
        },
    )
    .await
    .unwrap();

    password
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
    setup_admin(&env, admin_id).await;

    top_up(user_id, Amount(50), admin_id, &ctx).await.unwrap();
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
    let password = setup_admin(&env, admin_id).await;

    for _ in 0..5 {
        let _admin_token =
            issue_admin_pass(admin_id, password.clone(), &AuthPolicy::default(), &ctx)
                .await
                .unwrap();
        top_up(user_id, Amount(20), admin_id, &ctx).await.unwrap();
    }

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.balance, Amount(1000));
}

#[tokio::test]
async fn test_revoked_admin_cannot_top_up() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    let password = setup_admin(&env, admin_id).await;

    let user_id = setup_user(&env, "Alice", 100).await;

    assert!(top_up(user_id, Amount(10), admin_id, &ctx).await.is_ok());

    let root_id = UserId(Uuid::now_v7());
    setup_admin(&env, root_id).await;
    delta_core::services::auth::revoke_admin(root_id, admin_id, &ctx)
        .await
        .unwrap();

    let res = issue_admin_pass(admin_id, password, &AuthPolicy::default(), &ctx).await;
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
    setup_admin(&env, admin_id).await;

    let res = top_up(UserId(Uuid::now_v7()), Amount(100), admin_id, &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_spend_non_existent_user() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let res = spend(UserId(Uuid::now_v7()), Amount(100), &ctx).await;
    assert!(res.is_err());
}
