use crate::domain::{DomainError, UserId};
use crate::services::context::{HasAdmins, HasClock, HasTokens, HasUsers};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdminToken(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    SingleUse,
    Session,
}

pub struct TokenData {
    pub user_id: UserId,
    pub expires_at: DateTime<Utc>,
    pub kind: TokenKind,
}

pub fn issue_admin_pass<T>(
    user_id: UserId,
    password: String,
    ctx: &T,
) -> Result<AdminToken, DomainError>
where
    T: HasTokens + HasClock + HasUsers + HasAdmins,
{
    // check if user is admin
    // verify password against stored hash
    // issue single use token (e.g., JWT or random string)

    // NEEDS: UserRepo.get(user_id) -> Result<User, RepoError>
    //        AdminRepo.get(user_id) -> Result<Admin, RepoError>
    //        TokenRepo.insert(token) -> Result<(), RepoError>
    //        Clock.now() -> DateTime<Utc>

    todo!()
}

pub fn issue_admin_session<T>(token: AdminToken, ctx: &T) -> Result<AdminToken, DomainError>
where
    T: HasTokens + HasClock,
{
    // 1. validate token
    // 2. ensure NOT expired
    // 3. ensure single_use == true ← important
    // 4. delete the pass token ← mandatory
    // 5. issue multi-use token

    // NEEDS: TokenRepo.get(token) -> Result<AdminToken, RepoError>
    //        TokenRepo.insert(token) -> Result<(), RepoError>
    //        Clock.now() -> DateTime<Utc>
    todo!()
}

pub fn validate_authorization<T>(token: AdminToken, ctx: &T) -> Result<UserId, DomainError>
where
    T: HasTokens + HasClock,
{
    // validate token
    // return user id if valid
    // deletes expired tokens or single use tokens after validation

    // MUST:
    // - fail if token does not exist
    // - fail if expired
    // - decrement remaining_uses
    // - delete token if exhausted or expired
    // - return user_id on success

    // NEEDS: TokenRepo.get(token) -> Result<TokenRecord, RepoError>
    //        TokenRepo.delete() -> Result<(), RepoError>
    //        Clock.now() -> DateTime<Utc>
    todo!()
}
