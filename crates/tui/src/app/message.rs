use crate::api::models::auth::SingleUseToken;
use crate::api::models::user::UserId;
use crate::app::{Dialog, DialogOpenMode, Page};

#[derive(Debug)]
pub(crate) enum Message {
    Quit,

    Navigate(Page),

    CardScanned(String),

    Failed(AppError),

    Status(String),

    Request(Request),

    DialogOpen {
        dialog: Box<dyn Dialog>,
        mode: DialogOpenMode,
    },
    DialogClose,

    AdminAuthenticated(SingleUseToken),
}


#[derive(Debug)]
pub(crate) enum AppError {
    Api(String),
    Validation(String),
    Authentication(String),
    SessionExpired,
}

#[derive(Debug)]
pub(crate) enum Request {
    LookupUser(String),
    Spend { user_id: UserId, amount: u32 },
    TopUp { user_id: UserId, amount: u32 },
    AuthenticateAdmin { identifier: String, password: String },
}

