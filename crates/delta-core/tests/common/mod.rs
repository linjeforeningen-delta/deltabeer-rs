use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use delta_core::domain::{
    ActionRecord, AdminGrantId, Amount, PasswordHash, Role, Transaction, TransactionId, User,
    UserId,
};
use delta_core::infra::id::UuidIdGenerator;
use delta_core::infra::token::OpaqueTokenSource;
use delta_core::ports::Clock;
use delta_core::ports::repo::{AdminRepo, RepoError, TokenRepo, TransactionRepo, UserRepo};
use delta_core::services::auth::{AdminToken, TokenData};
use delta_core::services::context::Ctx;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct InMemoryRepo {
    pub users: Mutex<HashMap<UserId, User>>,
    pub admins: Mutex<HashMap<UserId, PasswordHash>>,
    pub tokens: Mutex<HashMap<[u8; 32], (TokenData, DateTime<Utc>)>>,
    pub transactions: Mutex<Vec<Transaction>>,
}

impl InMemoryRepo {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            admins: Mutex::new(HashMap::new()),
            tokens: Mutex::new(HashMap::new()),
            transactions: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl UserRepo for InMemoryRepo {
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
    async fn get_user_by_card(&self, card_number: u32) -> Result<User, RepoError> {
        self.users
            .lock()
            .unwrap()
            .values()
            .find(|u| u.card_number == card_number)
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
impl AdminRepo for InMemoryRepo {
    async fn get_admin(&self, id: UserId) -> Result<PasswordHash, RepoError> {
        self.admins
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(RepoError::NotFound)
    }
    async fn grant_admin(
        &self,
        _admin_grant_id: AdminGrantId,
        user_id: UserId,
        password_hash: PasswordHash,
        _record: ActionRecord,
    ) -> Result<(), RepoError> {
        self.admins.lock().unwrap().insert(user_id, password_hash);
        if let Some(user) = self.users.lock().unwrap().get_mut(&user_id) {
            user.role = Role::Admin;
        }
        Ok(())
    }
    async fn revoke_admin(&self, id: UserId, _record: ActionRecord) -> Result<(), RepoError> {
        self.admins.lock().unwrap().remove(&id);
        if let Some(user) = self.users.lock().unwrap().get_mut(&id) {
            user.role = Role::User;
        }
        Ok(())
    }
    async fn update_admin_password(
        &self,
        id: UserId,
        password_hash: PasswordHash,
    ) -> Result<(), RepoError> {
        self.admins.lock().unwrap().insert(id, password_hash);
        Ok(())
    }
}

#[async_trait]
impl TokenRepo for InMemoryRepo {
    async fn insert_token(
        &self,
        token: AdminToken,
        data: TokenData,
        created_at: DateTime<Utc>,
    ) -> Result<(), RepoError> {
        self.tokens
            .lock()
            .unwrap()
            .insert(token.0, (data, created_at));
        Ok(())
    }
    async fn get_token(
        &self,
        token: &AdminToken,
        dt: DateTime<Utc>,
    ) -> Result<Option<TokenData>, RepoError> {
        let res = self
            .tokens
            .lock()
            .unwrap()
            .get(&token.0)
            .map(|(data, _)| data)
            .filter(|data| data.expires_at > dt)
            .cloned();
        Ok(res)
    }
    async fn expire_token(&self, token: &AdminToken) -> Result<(), RepoError> {
        self.tokens.lock().unwrap().remove(&token.0);
        Ok(())
    }
}

#[async_trait]
impl TransactionRepo for InMemoryRepo {
    async fn spend(
        &self,
        tx_id: TransactionId,
        user_id: UserId,
        amount: Amount,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError> {
        let mut users = self.users.lock().unwrap();
        let user = users.get_mut(&user_id).ok_or(RepoError::NotFound)?;
        let updated_user = user
            .deduct_balance(amount)
            .map_err(|_| RepoError::NotFound)?;
        *user = updated_user;

        let tx = Transaction::Spend {
            id: tx_id,
            user_id,
            amount,
            ts,
        };
        self.transactions.lock().unwrap().push(tx.clone());
        Ok(tx)
    }
    async fn top_up(
        &self,
        tx_id: TransactionId,
        user_id: UserId,
        amount: Amount,
        approved_by: &UserId,
        ts: DateTime<Utc>,
    ) -> Result<Transaction, RepoError> {
        let mut users = self.users.lock().unwrap();
        let user = users.get_mut(&user_id).ok_or(RepoError::NotFound)?;
        let updated_user = user.add_balance(amount).map_err(|_| RepoError::NotFound)?;
        *user = updated_user;

        let tx = Transaction::TopUp {
            id: tx_id,
            user_id,
            amount,
            ts,
            approved_by: *approved_by,
        };
        self.transactions.lock().unwrap().push(tx.clone());
        Ok(tx)
    }
}

pub struct TestClock(pub DateTime<Utc>);
impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        self.0
    }
    fn today(&self) -> NaiveDate {
        self.0.date_naive()
    }
}

pub struct TestEnv {
    pub repo: InMemoryRepo,
    pub clock: TestClock,
    pub ids: UuidIdGenerator,
    pub tokens: OpaqueTokenSource,
}

impl TestEnv {
    pub fn new() -> Self {
        Self {
            repo: InMemoryRepo::new(),
            clock: TestClock(Utc::now()),
            ids: UuidIdGenerator,
            tokens: OpaqueTokenSource {},
        }
    }

    pub fn ctx(&self) -> Ctx<'_, InMemoryRepo> {
        Ctx {
            repo: &self.repo,
            token_repo: &self.repo,
            clock: &self.clock,
            ids: &self.ids,
            tokens: &self.tokens,
        }
    }
}
