use crate::domain::{ActionRecord, Amount, Role, User, UserId, UserIdent};
use crate::ports::RepoError;
use crate::services::context::{Ctx, HasClock, HasUsers};
use crate::services::ServiceError;
use chrono::NaiveDate;

pub async fn resolve_user<T>(ident: UserIdent, ctx: &T) -> Result<UserId, ServiceError>
where
    T: HasUsers,
{
    Ok(match ident {
        UserIdent::Id(id) => ctx.users().get(&id).await.map(|u| u.id)?,
        UserIdent::Card(card) => ctx.users().get_by_card(card).await.map(|u| u.id)?,
        UserIdent::Username(name) => ctx.users().get_by_name(&name).await.map(|u| u.id)?,
    })
}

pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub card_number: u32,
    pub birthdate: NaiveDate,
}

const MAX_RETRIES: usize = 3;

pub async fn create_user<T>(req: CreateUser, actor: UserId, ctx: &T) -> Result<UserId, ServiceError>
where
    T: HasUsers + HasClock,
{
    if !User::is_adult(req.birthdate, ctx.clock().today()) {
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
            at: ctx.clock().now(),
        };

        match ctx.users().insert(user, record).await {
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

pub async fn update_user(
    user_id: UserId,
    req: UpdateUser,
    ctx: &Ctx<'_>,
) -> Result<(), ServiceError> {
    let mut user = ctx.users().get(&user_id).await?;
    user.name = req.name.unwrap_or(user.name);
    user.username = req.username.unwrap_or(user.username);
    user.card_number = req.card_number.unwrap_or(user.card_number);
    ctx.users().update(user.clone()).await?;
    Ok(())
}
