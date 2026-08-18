use crate::api::models::auth::SingleUseToken;
use crate::api::models::transaction::Transaction;
use crate::api::models::user::{User, UserId};
use crate::app::{Dialog, DialogOpenMode, Page};
use chrono::NaiveDate;

#[derive(Debug)]
pub(crate) enum Message {
    ApiRequest(ApiRequest),
    ApiResponse(ApiResult),

    Status(String),
    Failed(AppError),

    OpenDialog {
        dialog: Box<dyn Dialog>,
        mode: DialogOpenMode,
    },
    CloseDialog,

    CardScanned(String),
    Navigate(Page),
    Quit,
}

#[derive(Debug)]
pub(crate) enum ApiResult {
    UserLoaded(User),
    SpendSucceeded(Transaction),
    TopUpSucceeded(Transaction),
    AdminAuthenticated(SingleUseToken),
    MakeUserSucceeded(User),
    RoleChanged(UserId),
}

#[derive(Debug)]
pub(crate) enum ApiRequest {
    LookupUser(String),
    Spend {
        user_id: UserId,
        amount: u32,
    },
    TopUp {
        user_id: UserId,
        amount: u32,
    },
    AuthenticateAdmin {
        identifier: String,
        password: String,
    },
    MakeUser {
        name: String,
        username: String,
        program: String,
        card_number: u32,
        birthdate: NaiveDate,
    },
    GrantAdmin {
        identifier: String,
        password: String,
    },
    RevokeAdmin {
        identifier: String,
    },
}

#[derive(Debug)]
pub(crate) enum AppError {
    Api(String),
    Validation(String),
    Authentication(String),
    SessionExpired,
}
