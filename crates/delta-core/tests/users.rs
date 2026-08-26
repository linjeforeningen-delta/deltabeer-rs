mod common;

use chrono::NaiveDate;
use common::TestEnv;
use delta_core::{
    domain::{Amount, Role, User, UserId},
    ports::repo::{AdminRepo, UserRepo},
    services::users::{CreateUser, UpdateUser, create_user, resolve_user, update_user},
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
async fn test_user_lifecycle() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    let req = CreateUser {
        name: "Alice".to_string(),
        username: "alice".to_string(),
        program: "Computer Science".to_string(),
        card_number: 111,
        birthdate: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
    };
    let user_id = create_user(admin_id, req, &ctx).await.unwrap();

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.name, "Alice");
    assert_eq!(user.balance, Amount(0));

    let up_req = UpdateUser {
        name: Some("Alice Updated".to_string()),
        username: None,
        program: None,
        card_number: Some(222),
        comments: Some("Updated comments".to_string()),
    };
    update_user(admin_id, user_id, up_req, &ctx).await.unwrap();

    let user = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(user.name, "Alice Updated");
    assert_eq!(user.card_number, 222);
    assert_eq!(user.comments, "Updated comments");
}

#[tokio::test]
async fn test_create_user_underage() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    let req = CreateUser {
        name: "Kid".to_string(),
        username: "kid".to_string(),
        program: "Computer Science".to_string(),
        card_number: 999,
        birthdate: env.clock.0.date_naive() - chrono::Duration::days(10 * 365),
    };
    let res = create_user(admin_id, req, &ctx).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn test_resolve_user_scenarios() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: "Bob".to_string(),
        username: "bob123".to_string(),
        program: "Computer Science".to_string(),
        card_number: 12345,
        role: Role::User,
        birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(0),
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

    assert_eq!(
        resolve_user(delta_core::domain::UserIdent::Id(user_id), &ctx)
            .await
            .unwrap(),
        user_id
    );
    assert_eq!(
        resolve_user(
            delta_core::domain::UserIdent::Username("bob123".to_string()),
            &ctx
        )
        .await
        .unwrap(),
        user_id
    );
    assert_eq!(
        resolve_user(delta_core::domain::UserIdent::Card(12345), &ctx)
            .await
            .unwrap(),
        user_id
    );
    assert!(
        resolve_user(delta_core::domain::UserIdent::Card(999), &ctx)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn test_update_user_partial() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    setup_admin(&env, admin_id, "admin").await;

    let user_id = UserId(Uuid::now_v7());
    let user = User {
        id: user_id,
        name: "Alice".to_string(),
        username: "alice".to_string(),
        program: "Old Program".to_string(),
        card_number: 100,
        role: Role::User,
        birthdate: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        comments: "".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    };
    UserRepo::insert_user(
        &env.repo,
        user,
        delta_core::domain::ActionRecord {
            actor: admin_id,
            at: env.clock.0,
        },
    )
    .await
    .unwrap();

    let up_req = UpdateUser {
        name: None,
        username: None,
        program: None,
        card_number: Some(500),
        comments: None,
    };
    update_user(admin_id, user_id, up_req, &ctx).await.unwrap();

    let updated = UserRepo::get_user(&env.repo, &user_id).await.unwrap();
    assert_eq!(updated.name, "Alice");
    assert_eq!(updated.card_number, 500);
}

// #[tokio::test]
// async fn test_create_user_unauthorized() {
//     let env = TestEnv::new();
//     let ctx = env.ctx();
//
//     let admin_id = UserId(Uuid::now_v7());
//
//     let req = CreateUser {
//         name: "Unauthorized".to_string(),
//         username: "unauth".to_string(),
//         card_number: 777,
//         birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
//     };
//     let res = create_user(admin_id, req, &ctx).await;
//     assert!(res.is_err());
// }

#[tokio::test]
async fn test_update_user_unauthorized() {
    let env = TestEnv::new();
    let ctx = env.ctx();

    let admin_id = UserId(Uuid::now_v7());
    let user_id = UserId(Uuid::now_v7());
    let up_req = UpdateUser {
        name: Some("New".to_string()),
        username: None,
        program: None,
        card_number: None,
        comments: None,
    };
    let res = update_user(admin_id, user_id, up_req, &ctx).await;
    assert!(res.is_err());
}
