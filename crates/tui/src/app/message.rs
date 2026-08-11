use crate::api::models::user::User;

#[derive(Debug)]
pub(crate) enum Message {
    Quit,

    OpenHome,
    OpenUsers,
    OpenTransactions,
    OpenStats,

    CardScanned(String),

    UserLoaded(User),
    UserLoadFailed(String),

    CloseDialog,
    NumericBackspace,
    Submit,
    NumericInput(char),
}

