use super::{AdminDialog, DialogBehavior, DialogResult, MenuDialog};
use crate::api::request::ApiRequest;
use crate::app::App;
use crate::app::dialog::menu::MenuOption;
use crate::app::{DialogOpenMode, Message};
use crate::auth::{AdminContext, AuthState};
use crate::ui::{dialogs::DialogView, theme::Theme};
use crossterm::event::KeyEvent;
use ratatui::Frame;

#[derive(Debug)]
pub(crate) struct AdminMenuDialog {
    pub(crate) menu: MenuDialog,
    pub(crate) logged_in: bool,
}

impl AdminMenuDialog {
    pub(crate) fn new() -> Self {
        let mut dialog = Self {
            menu: MenuDialog::new_admin("Admin", Vec::new()),
            logged_in: false,
        };
        dialog.refresh_auth_option();
        dialog
    }

    fn refresh_auth_option(&mut self) {
        self.menu.options = vec![
            MenuOption::new("Top Up", 'T', || Message::OpenDialog {
                dialog: Box::new(super::TopUpDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Make User", 'M', || Message::OpenDialog {
                dialog: Box::new(super::MakeUserDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Update User", 'U', || Message::OpenDialog {
                dialog: Box::new(super::UpdateUserDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Grant Admin", 'G', || Message::OpenDialog {
                dialog: Box::new(super::GrantAdminDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Revoke Admin", 'R', || Message::OpenDialog {
                dialog: Box::new(super::RevokeAdminDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            if self.logged_in {
                MenuOption::new("Logout", 'L', || {
                    Message::ApiRequest(ApiRequest::EndAdminSession)
                })
            } else {
                MenuOption::new("Login", 'L', || Message::OpenAdminDialog {
                    dialog: Box::new(super::AdminSessionLoginDialog::new()),
                    mode: DialogOpenMode::Push,
                })
            },
        ];
    }
}

impl DialogBehavior for AdminMenuDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        self.menu.handle_key_inner(key)
    }
}

impl DialogView for AdminMenuDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        self.menu.draw(frame, app, theme);
    }
}

impl AdminDialog for AdminMenuDialog {
    fn set_admin_context(&mut self, _context: Option<AdminContext>) {}

    fn set_auth_state(&mut self, state: &AuthState) {
        self.logged_in = matches!(state, AuthState::Admin(_));
        self.refresh_auth_option();
    }
}
