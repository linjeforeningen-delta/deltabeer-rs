mod stack;
mod user;
mod admin_auth;

use crate::app::TextInput;

pub(crate) use admin_auth::AdminAuthDialogState;
pub(crate) use stack::{DialogOpenMode, DialogStack};
pub(crate) use user::UserDialogState;
pub(crate) enum Dialog {
    User(UserDialogState),
    AdminAuth(AdminAuthDialogState),
}

impl Dialog {
    pub(crate) fn input_mut(&mut self) -> Option<&mut TextInput> {
        match self {
            Dialog::User(state) => Some(&mut state.amount),
            Dialog::AdminAuth(state) => Some(&mut state.password),
            _ => None,
        }
    }

    pub(crate) fn handle_scan(&mut self, card: String) -> Result<(), String> {
        match self {
            Dialog::AdminAuth(state) => state.handle_scan(card),
            _ => Err(card),
        }
    }
}
