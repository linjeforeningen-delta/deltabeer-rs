use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::dialog::AdminDialog;
use crate::app::dialog::menu::Language;
use crate::app::{Dialog, DialogOpenMode, Page};

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
    OpenAdminDialog {
        dialog: Box<dyn AdminDialog>,
        mode: DialogOpenMode,
    },
    CloseDialog,

    CardScanned(String),
    Navigate(Page),
    Quit,
    SetLanguage(Language),
    ToggleLanguage,
}

#[derive(Debug)]
pub(crate) enum AppError {
    Api(String),
    Validation(String),
    Authentication(String),
    SessionExpired,
}
