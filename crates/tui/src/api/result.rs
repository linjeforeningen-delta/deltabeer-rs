use crate::api::auth::{SessionToken, SingleUseToken};
use crate::model::{Transaction, User, UserId};

#[derive(Debug)]
pub(crate) enum ApiResult {
    LookupUser(User),
    Spend(Transaction),
    TopUp(Transaction),
    AuthenticateAdmin(SingleUseToken),
    StartAdminSession {
        user_id: UserId,
        token: SessionToken,
    },
    EndAdminSession,
    MakeUser(User),
    UpdateUser(User),
    GrantAdmin(UserId),
    RevokeAdmin(UserId),
}
