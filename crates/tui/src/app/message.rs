use crate::api::models::auth::SingleUseToken;
use crate::api::models::transaction::Transaction;
use crate::api::models::user::User;
use crate::app::Page;

#[derive(Debug)]
pub(crate) enum Message {
    Quit,

    Navigate(Page),

    CardScanned(String),

    Failed(AppError),

    User(UserMessage),
    Dialog(DialogMessage),
    Input(InputMessage),
    Transaction(TransactionMessage),
    Authentication(AuthenticationMessage),
}


#[derive(Debug)]
pub(crate) enum AppError {
    Api(String),
    Validation(String),
    Authentication(String),
    SessionExpired,
}


#[derive(Debug)]
pub(crate) enum UserMessage {
    Loaded(User),
    LoadFailed(String),
}

#[derive(Debug)]
pub(crate) enum DialogMessage {
    Close,
    TopUp,
}

#[derive(Debug)]
pub(crate) enum InputMessage {
    Char(char),
    Backspace,
    Submit,
}

#[derive(Debug)]
pub(crate) enum TransactionMessage {
    SpendSuccess(Transaction),
    SpendFailed(String),
    TopUpSuccess(Transaction),
    TopUpFailed(String),
}

#[derive(Debug)]
pub(crate) enum AuthenticationMessage {
    SingleUseToken(SingleUseToken),
    AdminAuthFailed(String),
}