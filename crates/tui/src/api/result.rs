use crate::api::models::auth::SingleUseToken;
use crate::api::models::transaction::Transaction;
use crate::api::models::user::{User, UserId};

#[derive(Debug)]
pub(crate) enum ApiResult {
    LookupUser(User),
    Spend(Transaction),
    TopUp(Transaction),
    AuthenticateAdmin(SingleUseToken),
    MakeUser(User),
    UpdateUser(User),
    GrantAdmin(UserId),
    RevokeAdmin(UserId),
}