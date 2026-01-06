use crate::domain::{ActionRecord, Amount, Role, User, UserId, UserIdent};
use crate::ports::repo::{TokenRepo, UserRepo};
use crate::services::auth::{validate_authorization, AdminToken};
use crate::services::context::Ctx;
use crate::services::ServiceError;
use chrono::NaiveDate;

pub async fn resolve_user<R>(ident: UserIdent, ctx: &Ctx<'_, R>) -> Result<UserId, ServiceError>
where
    R: UserRepo,
{
    Ok(match ident {
        UserIdent::Id(id) => ctx.repo.get_user(&id).await.map(|u| u.id)?,
        UserIdent::Card(card) => ctx.repo.get_user_by_card(card).await.map(|u| u.id)?,
        UserIdent::Username(name) => ctx.repo.get_user_by_name(&name).await.map(|u| u.id)?,
    })
}

pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub card_number: u32,
    pub birthdate: NaiveDate,
}

pub async fn create_user<R>(
    token: AdminToken,
    req: CreateUser,
    ctx: &Ctx<'_, R>,
) -> Result<UserId, ServiceError>
where
    R: UserRepo + TokenRepo,
{
    let actor = validate_authorization(token, ctx).await?;

    if !User::is_adult(req.birthdate, ctx.clock.today()) {
        return Err(ServiceError::Underage);
    }

    let dt = ctx.clock.now();
    let id = ctx.ids.generate_user_id();

    let user = User {
        id: id.clone(),
        name: req.name.clone(),
        username: req.username.clone(),
        card_number: req.card_number,
        role: Role::User,
        birthdate: req.birthdate,
        comments: "".to_string(),
        balance: Amount(0),
        spent: Amount(0),
    };

    let record = ActionRecord { actor, at: dt };

    ctx.repo.insert_user(user, record).await?;
    Ok(id)
}

pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub card_number: Option<u32>,
}

pub async fn update_user<R>(
    token: AdminToken,
    user_id: UserId,
    req: UpdateUser,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: UserRepo + TokenRepo,
{
    let _actor = validate_authorization(token, ctx).await?;

    let mut user = ctx.repo.get_user(&user_id).await?;
    user.name = req.name.unwrap_or(user.name);
    user.username = req.username.unwrap_or(user.username);
    user.card_number = req.card_number.unwrap_or(user.card_number);
    ctx.repo.update_user(user.clone()).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{ActionRecord, Amount, Role, User, UserId};
    use crate::ports::repo::{RepoError, TokenRepo, UserRepo};
    use crate::ports::{Clock, IdGenerator, TokenSource};
    use crate::services::auth::{AdminToken, TokenData, TokenKind};
    use crate::services::context::Ctx;
    use async_trait::async_trait;
    use chrono::{DateTime, NaiveDate, Utc};
    use std::sync::Mutex;
    use uuid::Uuid;

    struct MockRepo {
        users: Mutex<std::collections::HashMap<UserId, User>>,
    }

    #[async_trait]
    impl UserRepo for MockRepo {
        async fn get_user(&self, key: &UserId) -> Result<User, RepoError> {
            self.users
                .lock()
                .unwrap()
                .get(key)
                .cloned()
                .ok_or(RepoError::NotFound)
        }
        async fn get_user_by_name(&self, name: &str) -> Result<User, RepoError> {
            self.users
                .lock()
                .unwrap()
                .values()
                .find(|u| u.username == name)
                .cloned()
                .ok_or(RepoError::NotFound)
        }
        async fn get_user_by_card(&self, card: u32) -> Result<User, RepoError> {
            self.users
                .lock()
                .unwrap()
                .values()
                .find(|u| u.card_number == card)
                .cloned()
                .ok_or(RepoError::NotFound)
        }
        async fn insert_user(&self, user: User, _record: ActionRecord) -> Result<(), RepoError> {
            self.users.lock().unwrap().insert(user.id, user);
            Ok(())
        }
        async fn update_user(&self, user: User) -> Result<(), RepoError> {
            self.users.lock().unwrap().insert(user.id, user);
            Ok(())
        }
    }

    #[async_trait]
    impl TokenRepo for MockRepo {
        async fn insert_token(
            &self,
            _token: AdminToken,
            _data: TokenData,
            _created_at: DateTime<Utc>,
        ) -> Result<(), RepoError> {
            Ok(())
        }
        async fn get_token(
            &self,
            _token: &AdminToken,
            _dt: DateTime<Utc>,
        ) -> Result<Option<TokenData>, RepoError> {
            Ok(None)
        }
        async fn expire_token(&self, _token: &AdminToken) -> Result<(), RepoError> {
            Ok(())
        }
    }

    struct MockClock(DateTime<Utc>);
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            self.0
        }
        fn today(&self) -> NaiveDate {
            self.0.date_naive()
        }
    }

    struct MockIds;
    impl IdGenerator for MockIds {
        fn generate_user_id(&self) -> UserId {
            UserId(Uuid::nil())
        }
        fn generate_transaction_id(&self) -> crate::domain::TransactionId {
            crate::domain::TransactionId(Uuid::nil())
        }
        fn generate_admin_grant_id(&self) -> crate::domain::AdminGrantId {
            crate::domain::AdminGrantId(Uuid::nil())
        }
    }

    struct MockTokens;
    #[async_trait(?Send)]
    impl TokenSource for MockTokens {
        async fn issue_token(
            &self,
            _user_id: UserId,
            _ttl: chrono::Duration,
            _kind: TokenKind,
            _repo: &dyn TokenRepo,
            _clock: &dyn Clock,
        ) -> Result<AdminToken, crate::ports::TokenError> {
            Ok(AdminToken([0; 32]))
        }
        async fn expire_token(
            &self,
            _token: AdminToken,
            _repo: &dyn TokenRepo,
        ) -> Result<(), crate::ports::TokenError> {
            Ok(())
        }
        async fn validate_token(
            &self,
            _token: AdminToken,
            _repo: &dyn TokenRepo,
            _clock: &dyn Clock,
        ) -> Result<UserId, crate::ports::TokenError> {
            Ok(UserId(Uuid::nil()))
        }
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let repo = MockRepo {
            users: Mutex::new(std::collections::HashMap::new()),
        };
        let clock = MockClock(Utc::now());
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let req = CreateUser {
            name: "John Doe".to_string(),
            username: "jdoe".to_string(),
            card_number: 12345,
            birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        };

        let admin_token = AdminToken([0; 32]);
        let res = create_user(admin_token, req, &ctx).await;
        assert!(res.is_ok());
        let user_id = res.unwrap();

        let user = repo.get_user(&user_id).await.unwrap();
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.username, "jdoe");
    }

    #[tokio::test]
    async fn test_create_user_underage() {
        let repo = MockRepo {
            users: Mutex::new(std::collections::HashMap::new()),
        };
        let clock = MockClock(DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2024, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        ));
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let req = CreateUser {
            name: "Young One".to_string(),
            username: "young".to_string(),
            card_number: 54321,
            birthdate: NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
        };

        let admin_token = AdminToken([0; 32]);
        let res = create_user(admin_token, req, &ctx).await;
        assert!(matches!(res, Err(ServiceError::Underage)));
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let user_id = UserId(Uuid::nil());
        let user = User {
            id: user_id,
            name: "Old Name".to_string(),
            username: "old".to_string(),
            card_number: 1,
            role: Role::User,
            birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            comments: "".to_string(),
            balance: Amount(0),
            spent: Amount(0),
        };
        let mut users = std::collections::HashMap::new();
        users.insert(user_id, user);

        let repo = MockRepo {
            users: Mutex::new(users),
        };
        let clock = MockClock(Utc::now());
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let req = UpdateUser {
            name: Some("New Name".to_string()),
            username: None,
            card_number: Some(999),
        };

        let admin_token = AdminToken([0; 32]);
        let res = update_user(admin_token, user_id, req, &ctx).await;
        assert!(res.is_ok());

        let updated_user = repo.get_user(&user_id).await.unwrap();
        assert_eq!(updated_user.name, "New Name");
        assert_eq!(updated_user.username, "old");
        assert_eq!(updated_user.card_number, 999);
    }
}
