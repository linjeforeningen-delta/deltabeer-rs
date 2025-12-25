use crate::domain::{DomainError, UserId};
use crate::ports::{AdminRepo, Clock, TokenRepo, UserRepo};
use chrono::{DateTime, Utc};

struct Ctx<'a> {
    users: &'a dyn UserRepo,
    admins: &'a dyn AdminRepo,
    clock: &'a dyn Clock,
    token: &'a dyn TokenRepo,
}

pub struct AdminToken(pub String);

enum TokenKind {
    SingleUse,
    Session,
}

pub struct TokenData {
    user_id: UserId,
    expires_at: DateTime<Utc>,
    kind: TokenKind,
}

fn issue_admin_pass(
    user_id: UserId,
    password: String,
    ctx: &Ctx<'_>,
) -> Result<AdminToken, DomainError> {
    // check if user is admin
    // verify password against stored hash
    // issue single use token (e.g., JWT or random string)

    // NEEDS: UserRepo.get(user_id) -> Result<User, RepoError>
    //        AdminRepo.get(user_id) -> Result<Admin, RepoError>
    //        TokenRepo.insert(token) -> Result<(), RepoError>
    //        Clock.now() -> DateTime<Utc>

    todo!()
}

fn issue_admin_session(token: AdminToken, ctx: &Ctx<'_>) -> Result<AdminToken, DomainError> {
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

fn validate_authorization(token: AdminToken, ctx: &Ctx<'_>) -> Result<UserId, DomainError> {
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
