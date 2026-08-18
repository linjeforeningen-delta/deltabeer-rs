use crate::api::command::ApiCommand;
use crate::api::models::auth::SingleUseToken;
use crate::api::request::ApiRequest;
use crate::app::DialogOpenMode;
use crate::app::dialog::AdminAuthDialog;
use crate::app::dialog::DialogStack;
use crate::auth::{AdminContext, AuthState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Home,
    Users,
    Transactions,
    Stats,
}

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
            page: Page::Home,
            dialogs: DialogStack::new(),
            status: "Ready for card".into(),
            pending_api_request: None,
            active_admin: None,
            should_quit: false,
        }
    }

    pub(crate) fn request_api(
        &mut self,
        request: ApiRequest,
    ) -> Option<ApiCommand> {
        if !request.requires_auth() {
            return Some(ApiCommand {
                request,
                authorization: None,
            });
        }

        match &self.auth {
            AuthState::Admin(session) => Some(ApiCommand {
                request,
                authorization: Some(
                    session.token.clone().into(),
                ),
            }),

            AuthState::Normal => {
                let admin = self.active_admin.clone();

                self.pending_api_request = Some(request);

                self.dialogs.open(Box::new(AdminAuthDialog::new(admin)), DialogOpenMode::Push);
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
