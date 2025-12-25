use crate::domain::{DomainError, Role, UserId};
use crate::ports::{AdminRepo, Clock, UserRepo};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

struct Ctx<'a> {
    users: &'a dyn UserRepo,
    admins: &'a dyn AdminRepo,
    clock: &'a dyn Clock,
}

pub struct Ident<'a>(pub &'a str);
fn resolve_user(ident: Ident<'_>, ctx: &Ctx<'_>) -> Result<UserId, DomainError> {
    // find user by ident
    // parse ident to correct type
    // match type:
    //   username -> resolve username to user id
    //   card -> resolve card number to user id
    //   id -> return user id
    // NEEDS: UserRepo.get_by_username(username) -> Result<User, RepoError>
    //        UserRepo.get_by_card(card_number) -> Result<User, RepoError>
    //        UserRepo.get(id) -> Result<User, RepoError>
    todo!()
}

pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub card_number: u32,
    pub birthdate: NaiveDate,
}

pub async fn create_user(req: CreateUser, ctx: &Ctx<'_>) -> Result<UserId, DomainError> {
    // Create a new user in the system
    // Validate username uniqueness
    // Validate card number uniqueness
    // Validate birthdate (user is adult)
    // Sets defaults for other fields
    // Persist user
    // defaults:
    //   role = User
    //   balance = 0
    //   spent = 0
    //   comments = ""
    // NEEDS: UserRepo.insert(user) -> Result<UserId, RepoError>
    //        UserRepo.get(id) -> Result<User, RepoError>
    //        UserRepo.get_by_username(username) -> Result<User, RepoError>
    //        UserRepo.get_by_card(card_number) -> Result<User, RepoError>
    todo!()
}

pub struct PartialUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub card_number: Option<u32>,
}

pub async fn update_user(
    user_id: UserId,
    req: PartialUser,
    ctx: &Ctx<'_>,
) -> Result<UserId, DomainError> {
    // get user by id
    // update user fields
    // validate username uniqueness
    // validate card number uniqueness
    // persist user
    // NEEDS: UserRepo.update(user) -> Result<(), RepoError>
    //        UserRepo.get(id) -> Result<User, RepoError>
    //        UserRepo.get_by_username(username) -> Result<User, RepoError>
    //        UserRepo.get_by_card(card_number) -> Result<User, RepoError>
    todo!()
}

pub async fn change_user_role(
    user_id: UserId,
    new_role: Role,
    admin_password: Option<String>,
    ctx: &Ctx<'_>,
) -> Result<(), DomainError> {
    // get user by id
    // parse new_role to Role enum
    // update user role
    // if change to admin, persist in AdminRepo with password as well
    // persist user
    // NEEDS: UserRepo.update(user) -> Result<(), RepoError>
    //        UserRepo.get(id) -> Result<User, RepoError>
    //        AdminRepo.insert(admin) -> Result<(), RepoError>
    //        AdminRepo.delete(user_id) -> Result<(), RepoError>
    todo!()
}
