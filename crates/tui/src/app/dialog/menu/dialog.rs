use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Debug;

use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{AdminDialog, DialogBehavior, DialogResult};
use crate::auth::{AdminContext, AuthState};

#[derive(Debug)]
pub(crate) struct MenuDialog {
    pub(crate) title: MenuTitle,
    pub(crate) options: Vec<MenuOption>,
    pub(crate) kind: MenuKind,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MenuTitle {
    Admin,
    Application,
    Language,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MenuKind {
    Normal,
    Admin { logged_in: bool },
}

impl MenuDialog {
    pub(crate) fn new(title: MenuTitle, options: Vec<MenuOption>, kind: MenuKind) -> Self {
        Self {
            title,
            options,
            kind,
        }
    }
}

impl AdminDialog for MenuDialog {
    fn set_admin_context(&mut self, _context: Option<AdminContext>) {}

    fn set_auth_state(&mut self, state: &AuthState) {
        let MenuKind::Admin { logged_in } = &mut self.kind else {
            return;
        };

        let next_logged_in = matches!(state, AuthState::Admin(_));
        if *logged_in != next_logged_in {
            *logged_in = next_logged_in;
            self.options = crate::app::dialog::menu::preset::admin::options(next_logged_in);
        }
    }
}

impl DialogBehavior for MenuDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        let KeyCode::Char(char) = key.code else {
            return DialogResult::Unhandled(key);
        };

        let Some(option) = self
            .options
            .iter()
            .find(|option| option.key.eq_ignore_ascii_case(&char))
        else {
            return DialogResult::Unhandled(key);
        };

        DialogResult::Message((option.message)())
    }
}
