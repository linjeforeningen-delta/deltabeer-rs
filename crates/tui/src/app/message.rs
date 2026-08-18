use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
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
    CloseDialog,

    CardScanned(String),
    Navigate(Page),
    Quit,
}

#[derive(Debug)]
pub(crate) enum AppError {
    Api(String),
    Validation(String),
    Authentication(String),
    SessionExpired,
}
