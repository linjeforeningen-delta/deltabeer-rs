use crate::domain::{Amount, Transaction, UserId};
use crate::ports::repo::{TokenRepo, TransactionRepo, UserRepo};
use crate::services::auth::validate_authorization;
use crate::services::context::Ctx;
use crate::services::{auth::AdminToken, ServiceError};

pub async fn spend<R>(
    user_id: UserId,
    amount: Amount,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo,
{
    let tx_id = ctx.ids.generate_transaction_id();
    let tx = ctx
        .repo
        .spend(tx_id, user_id, amount, ctx.clock.now())
        .await?;
    Ok(tx)
}

pub async fn top_up<R>(
    user_id: UserId,
    amount: Amount,
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<Transaction, ServiceError>
where
    R: TransactionRepo + UserRepo + TokenRepo,
{
    let admin_id = validate_authorization(token, ctx).await?;
    let tx_id = ctx.ids.generate_transaction_id();

    let tx = ctx
        .repo
        .top_up(tx_id, user_id, amount, &admin_id, ctx.clock.now())
        .await?;
    Ok(tx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Amount, Transaction, TransactionId, UserId};
    use crate::ports::repo::{RepoError, TokenRepo, TransactionRepo, UserRepo};
    use crate::ports::{Clock, IdGenerator, TokenSource};
    use crate::services::auth::{AdminToken, TokenData, TokenKind};
    use crate::services::context::Ctx;
    use async_trait::async_trait;
    use chrono::{DateTime, NaiveDate, Utc};
    use uuid::Uuid;

    struct MockRepo;

    #[async_trait]
    impl UserRepo for MockRepo {
        async fn get_user(&self, _key: &UserId) -> Result<crate::domain::User, RepoError> {
            Err(RepoError::NotFound)
        }
        async fn get_user_by_name(&self, _name: &str) -> Result<crate::domain::User, RepoError> {
            Err(RepoError::NotFound)
        }
        async fn get_user_by_card(&self, _card: u32) -> Result<crate::domain::User, RepoError> {
            Err(RepoError::NotFound)
        }
        async fn insert_user(
            &self,
            _user: crate::domain::User,
            _record: crate::domain::ActionRecord,
        ) -> Result<(), RepoError> {
            Ok(())
        }
        async fn update_user(&self, _user: crate::domain::User) -> Result<(), RepoError> {
            Ok(())
        }
    }

    #[async_trait]
    impl TransactionRepo for MockRepo {
        async fn spend(
            &self,
            tx_id: TransactionId,
            user: UserId,
            amount: Amount,
            ts: DateTime<Utc>,
        ) -> Result<Transaction, RepoError> {
            if amount.0 > 1000 {
                return Err(RepoError::NotFound); // Simulate error
            }
            Ok(Transaction::Spend {
                id: tx_id,
                user_id: user,
                amount,
                ts,
            })
        }
        async fn top_up(
            &self,
            tx_id: TransactionId,
            user: UserId,
            amount: Amount,
            approved_by: &UserId,
            ts: DateTime<Utc>,
        ) -> Result<Transaction, RepoError> {
            Ok(Transaction::TopUp {
                id: tx_id,
                user_id: user,
                amount,
                ts,
                approved_by: *approved_by,
            })
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
        fn generate_transaction_id(&self) -> TransactionId {
            TransactionId(Uuid::nil())
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
    async fn test_spend_success() {
        let repo = MockRepo;
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let user_id = UserId(Uuid::nil());
        let res = spend(user_id, Amount(50), &ctx).await;
        assert!(res.is_ok());
        let tx = res.unwrap();
        match tx {
            Transaction::Spend { amount, .. } => assert_eq!(amount, Amount(50)),
            _ => panic!("Expected Spend transaction"),
        }
    }

    #[tokio::test]
    async fn test_spend_fail() {
        let repo = MockRepo;
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let user_id = UserId(Uuid::nil());
        let res = spend(user_id, Amount(2000), &ctx).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_top_up_success() {
        let repo = MockRepo;
        let clock = MockClock;
        let ids = MockIds;
        let tokens = MockTokens;
        let ctx = Ctx {
            repo: &repo,
            clock: &clock,
            ids: &ids,
            tokens: &tokens,
        };

        let user_id = UserId(Uuid::nil());
        let admin_token = AdminToken([0; 32]);
        let res = top_up(user_id, Amount(100), admin_token, &ctx).await;
        assert!(res.is_ok());
        let tx = res.unwrap();
        match tx {
            Transaction::TopUp {
                amount,
                approved_by,
                ..
            } => {
                assert_eq!(amount, Amount(100));
                assert_eq!(approved_by, UserId(Uuid::nil()));
            }
            _ => panic!("Expected TopUp transaction"),
        }
    }
}
