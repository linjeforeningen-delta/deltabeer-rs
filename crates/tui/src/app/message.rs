use crate::api::{request::ApiRequest, result::ApiResult};
use crate::app::dialog::AdminDialog;
use crate::app::dialog::menu::Language;
use crate::app::{Dialog, DialogOpenMode, PageId};

use super::{error::AppError, status::StatusMessage};

#[derive(Debug)]
pub(crate) enum Message {
    ApiRequest(ApiRequest),
    ApiResponse(ApiResult),

    Status(StatusMessage),
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
    Navigate(PageId),
    Quit,
    SetLanguage(Language),
    ToggleLanguage,
}
