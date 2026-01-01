use crate::domain::{ActionRecord, Amount, Role, User, UserId, UserIdent};
use crate::ports::{RepoError, UserRepo};
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

const MAX_RETRIES: usize = 3;

pub async fn create_user<R>(
    req: CreateUser,
    actor: UserId,
    ctx: &Ctx<'_, R>,
) -> Result<UserId, ServiceError>
where
    R: UserRepo,
{
    if !User::is_adult(req.birthdate, ctx.clock.today()) {
        return Err(ServiceError::Underage);
    }

    for _ in 0..MAX_RETRIES {
        let id = UserId::new();

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

        let record = ActionRecord {
            actor,
            at: ctx.clock.now(),
        };

        match ctx.repo.insert_user(user, record).await {
            Ok(()) => return Ok(id),
            Err(RepoError::Conflict) => continue,
            Err(e) => return Err(ServiceError::from(e)),
        }
    }
    Err(ServiceError::Conflict)
}

pub struct UpdateUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub card_number: Option<u32>,
}

pub async fn update_user<R>(
    user_id: UserId,
    req: UpdateUser,
    ctx: &Ctx<'_, R>,
) -> Result<(), ServiceError>
where
    R: UserRepo,
{
    let mut user = ctx.repo.get_user(&user_id).await?;
    user.name = req.name.unwrap_or(user.name);
    user.username = req.username.unwrap_or(user.username);
    user.card_number = req.card_number.unwrap_or(user.card_number);
    ctx.repo.update_user(user.clone()).await?;
    Ok(())
}
