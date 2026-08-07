use crate::domain::{ActionRecord, PasswordCheck, UserId, hash_password, verify_password};
use crate::ports::repo::{AdminRepo, TokenRepo, UserRepo};
use crate::services::ServiceError;
use crate::services::context::Ctx;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminToken(pub [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    SingleUse,
    Session,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenData {
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub kind: TokenKind,
}

pub async fn login<R>(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: AdminRepo + ?Sized,
{
    let hash = ctx.repo.get_admin(user_id).await?;
    match verify_password(&password, &hash)? {
        PasswordCheck::VerifiedAndNeedsRehash => {
            update_password(user_id, password, ctx).await?;
        }
        PasswordCheck::Verified => {}
    }
    Ok(())
}

pub async fn issue_admin_pass<R>(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, ServiceError>
where
    R: AdminRepo + UserRepo + ?Sized,
{
    login(user_id, password, ctx).await?;
    let ttl = chrono::Duration::seconds(15);
    let token = ctx
        .tokens
        .issue_token(user_id, ttl, TokenKind::SingleUse, ctx.token_repo, ctx.clock)
        .await?;

    Ok(token)
}

pub async fn issue_admin_session<R>(
    actor: UserId,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, ServiceError>
where
    R: ?Sized,
{
    let new_token = ctx
        .tokens
        .issue_token(
            actor,
            chrono::Duration::minutes(3),
            TokenKind::Session,
            ctx.token_repo,
            ctx.clock,
        )
        .await?;
    Ok(new_token)
}

pub async fn validate_authorization<R>(
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<UserId, ServiceError>
where
    R: ?Sized,
{
    Ok(ctx
        .tokens
        .validate_token(token, ctx.token_repo, ctx.clock)
        .await?)
}

pub async fn grant_admin<R>(
    actor: UserId,
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: TokenRepo + AdminRepo,
{
    let now = ctx.clock.now();
    let hash = hash_password(&*password);

    let grant_id = ctx.ids.generate_admin_grant_id();

    let record = ActionRecord { actor, at: now };

    ctx.repo
        .grant_admin(grant_id, user_id, hash, record)
        .await?;
    Ok(())
}

pub async fn revoke_admin<R>(
    actor: UserId,
    user_id: UserId,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: TokenRepo + AdminRepo,
{
    let now = ctx.clock.now();

    let record = ActionRecord { actor, at: now };
    ctx.repo.revoke_admin(user_id, record).await?;
    Ok(())
}

pub async fn update_password<R>(
    user_id: UserId,
    new_password: String,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: AdminRepo + ?Sized,
{
    let hash = hash_password(&*new_password);

    ctx.repo.update_admin_password(user_id, hash).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Amount, Role, User, UserId};
    use crate::ports::repo::{AdminRepo, RepoError, TokenRepo, UserRepo};
    use crate::ports::{Clock, IdGenerator, TokenSource};
    use crate::services::context::Ctx;
    use async_trait::async_trait;
    use chrono::{DateTime, NaiveDate, Utc};
    use std::sync::Mutex;
    use uuid::Uuid;

    struct MockRepo {
        admins: Mutex<std::collections::HashMap<UserId, String>>,
    }

    #[async_trait]
    impl UserRepo for MockRepo {
        async fn get_user(&self, key: &UserId) -> Result<User, RepoError> {
            Ok(User {
                id: *key,
                name: "".to_string(),
                username: "".to_string(),
                card_number: 0,
                role: Role::Admin,
                birthdate: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                comments: "".to_string(),
                balance: Amount(0),
                spent: Amount(0),
            })
        }
        async fn get_user_by_name(&self, _name: &str) -> Result<User, RepoError> {
            Err(RepoError::NotFound)
        }
        async fn get_user_by_card(&self, _card: u32) -> Result<User, RepoError> {
            Err(RepoError::NotFound)
        }
        async fn list_users(&self) -> Result<Vec<User>, RepoError> {
            Ok(Vec::new())
        }
        async fn list_admins(&self) -> Result<Vec<User>, RepoError> {
            Ok(Vec::new())
        }
        async fn insert_user(
            &self,
            _user: User,
            _record: crate::domain::ActionRecord,
        ) -> Result<(), RepoError> {
            Ok(())
        }
        async fn update_user(&self, _user: User) -> Result<(), RepoError> {
            Ok(())
        }
    }

    #[async_trait]
    impl AdminRepo for MockRepo {
        async fn get_admin(&self, id: UserId) -> Result<crate::domain::PasswordHash, RepoError> {
            self.admins
                .lock()
                .unwrap()
                .get(&id)
                .map(|h| crate::domain::PasswordHash(h.clone()))
                .ok_or(RepoError::NotFound)
        }
        async fn grant_admin(
            &self,
            _admin_grant_id: crate::domain::AdminGrantId,
            user_id: UserId,
            password_hash: crate::domain::PasswordHash,
            _record: crate::domain::ActionRecord,
        ) -> Result<(), RepoError> {
            self.admins.lock().unwrap().insert(user_id, password_hash.0);
            Ok(())
        }
        async fn revoke_admin(
            &self,
            id: UserId,
            _record: crate::domain::ActionRecord,
        ) -> Result<(), RepoError> {
            self.admins.lock().unwrap().remove(&id);
            Ok(())
        }
        async fn update_admin_password(
            &self,
            id: UserId,
            password_hash: crate::domain::PasswordHash,
        ) -> Result<(), RepoError> {
            self.admins.lock().unwrap().insert(id, password_hash.0);
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

    struct MockClock;
    impl Clock for MockClock {
        fn now(&self) -> DateTime<Utc> {
            Utc::now()
        }
        fn today(&self) -> NaiveDate {
            Utc::now().date_naive()
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
    #[async_trait]
    impl TokenSource for MockTokens {
        async fn issue_token(
            &self,
            _user_id: UserId,
            _ttl: chrono::Duration,
            _kind: TokenKind,
            _repo: &(dyn TokenRepo + Sync),
            _clock: &(dyn Clock + Sync),
        ) -> Result<AdminToken, crate::ports::TokenError> {
            Ok(AdminToken([0; 32]))
        }
        async fn expire_token(
            &self,
            _token: AdminToken,
            _repo: &(dyn TokenRepo + Sync),
        ) -> Result<(), crate::ports::TokenError> {
            Ok(())
        }
        async fn validate_token(
            &self,
            _token: AdminToken,
            _repo: &(dyn TokenRepo + Sync),
            _clock: &(dyn Clock + Sync),
        ) -> Result<UserId, crate::ports::TokenError> {
            Ok(UserId(Uuid::nil()))
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        let user_id = UserId(Uuid::nil());
        let password = "password123";
        let hash = hash_password(password);

        let mut admins = std::collections::HashMap::new();
        admins.insert(user_id, hash.0);

        let repo = MockRepo {
            admins: Mutex::new(admins),
        };
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            token_repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let res = login(user_id, password.to_string(), &ctx).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let user_id = UserId(Uuid::nil());
        let password = "password123";
        let hash = hash_password(password);

        let mut admins = std::collections::HashMap::new();
        admins.insert(user_id, hash.0);

        let repo = MockRepo {
            admins: Mutex::new(admins),
        };
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            token_repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let res = login(user_id, "wrong".to_string(), &ctx).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_grant_admin() {
        let repo = MockRepo {
            admins: Mutex::new(std::collections::HashMap::new()),
        };
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            token_repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let actor_id = UserId(Uuid::nil());
        let user_id = UserId(Uuid::nil());
        let res = grant_admin(actor_id, user_id, "newpass".to_string(), &ctx).await;
        assert!(res.is_ok());

        assert!(repo.admins.lock().unwrap().contains_key(&user_id));
    }
}
