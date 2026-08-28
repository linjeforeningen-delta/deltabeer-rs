use crate::api::{request::ApiRequest, result::ApiResult};
use crate::app::dialog::AdminDialog;
use crate::app::dialog::menu::Language;
use crate::app::{Dialog, DialogOpenMode, PageId};

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
    Navigate(PageId),
    Quit,
    SetLanguage(Language),
    ToggleLanguage,
}

#[derive(Debug)]
pub(crate) enum AppError {
    /// The API returned an error that has no distinct TUI behavior.
    Api,
    Unauthorized,
    Forbidden,
    NotFound,
    InvalidUserIdentifier,
    Conflict,
    BadRequest,
    Transport,
    InvalidResponse,
    Validation(String),
    MissingAuthorization {
        operation: AuthorizationOperation,
    },
    SessionExpired,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum AuthorizationOperation {
    TopUp,
    EndAdminSession,
    CreateUser,
    UpdateUser,
    GrantAdmin,
    RevokeAdmin,
}
