use crate::api::auth::SingleUseToken;
use crate::api::command::ApiCommand;
use crate::api::request::ApiRequest;
use crate::app::DialogOpenMode;
use crate::app::dialog::AdminAuthDialog;
use crate::app::dialog::DialogStack;
use crate::app::page::{Page, PageId};
use crate::auth::{AdminContext, AuthState};

pub(crate) struct App {
    pub auth: AuthState,
    pub page: Page,
    pub dialogs: DialogStack,
    pub status: String,
    pending_api_request: Option<ApiRequest>,
    pub(crate) active_admin: Option<AdminContext>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            page: Page::from(PageId::Home),
            dialogs: DialogStack::new(),
            status: t!("status.ready").to_string(),
            pending_api_request: None,
            active_admin: None,
            should_quit: false,
        }
    }

    pub(crate) fn request_api(&mut self, request: ApiRequest) -> Option<ApiCommand> {
        if matches!(request, ApiRequest::EndAdminSession) {
            let AuthState::Admin(session) = &self.auth else {
                return None;
            };

            return Some(ApiCommand {
                request,
                authorization: Some(session.token.clone().into()),
            });
        }

        if !request.requires_auth() {
            return Some(ApiCommand {
                request,
                authorization: None,
            });
        }

        match &self.auth {
            AuthState::Admin(session) => Some(ApiCommand {
                request,
                authorization: Some(session.token.clone().into()),
            }),

            AuthState::Normal => {
                let admin = self.active_admin.clone();

                self.pending_api_request = Some(request);

                self.dialogs
                    .open(Box::new(AdminAuthDialog::new(admin)), DialogOpenMode::Push);
                None
            }
        }
    }

    pub(crate) fn complete_pending_request(&mut self, token: SingleUseToken) -> Option<ApiCommand> {
        let request = self.pending_api_request.take()?;

        self.dialogs.close();

        Some(ApiCommand {
            request,
            authorization: Some(token.into()),
        })
    }
}
