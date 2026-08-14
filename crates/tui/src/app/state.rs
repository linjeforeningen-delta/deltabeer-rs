use crate::api::models::auth::SingleUseToken;
use crate::app::admin_action::AdminAction;
use crate::app::dialog::AdminAuthDialog;
use crate::app::dialog::DialogStack;
use crate::app::{Command, DialogOpenMode, Message};
use crate::auth::AuthState;

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
    pending_admin_action: Option<AdminAction>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            page: Page::Home,
            dialogs: DialogStack::new(),
            status: "Ready for card".into(),
            pending_admin_action: None,
            should_quit: false,
        }
    }

    pub(crate) fn request_admin_action(
        &mut self,
        action: AdminAction,
    ) -> Option<Command> {
        match &self.auth {
            AuthState::Admin(session) => {
                return Some(
                    action.into_command(session.token.clone().into())
                );
            }

            AuthState::Normal => {
                self.pending_admin_action = Some(action);

                self.update(
                    Message::OpenDialog {
                        dialog: Box::new(AdminAuthDialog::default()),
                        mode: DialogOpenMode::Push,
                    })
            }
        }
    }

    pub(crate) fn complete_admin_auth(
        &mut self,
        token: SingleUseToken,
    ) -> Option<Command> {
        let action = self.pending_admin_action.take()?;

        self.dialogs.close();

        Some(
            action.into_command(
                token.into()
            )
        )
    }
}