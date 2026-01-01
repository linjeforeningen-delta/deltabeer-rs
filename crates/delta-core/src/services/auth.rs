use crate::domain::{DomainError, UserId};
use crate::ports::{AdminRepo, TokenRepo, UserRepo};
use crate::services::context::Ctx;
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

pub fn issue_admin_pass<R>(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, DomainError>
where
    R: TokenRepo + AdminRepo + UserRepo,
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

pub fn issue_admin_session<R>(
    token: AdminToken,
    ctx: &Ctx<'_, R>,
) -> Result<AdminToken, DomainError>
where
    R: TokenRepo,
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

pub fn validate_authorization<R>(token: AdminToken, ctx: &Ctx<'_, R>) -> Result<UserId, DomainError>
where
    R: TokenRepo,
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
