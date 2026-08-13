mod stack;
mod user;

use crate::app::TextInput;

pub(crate) use stack::{DialogOpenMode, DialogStack};
pub(crate) use user::UserDialogState;

pub(crate) enum Dialog {
    User(UserDialogState),
}

impl Dialog {
    pub(crate) fn input_mut(&mut self) -> Option<&mut TextInput> {
        match self {
            Dialog::User(state) => Some(&mut state.amount),
            _ => None,
        }
    }

    pub(crate) fn handle_scan(&mut self, card: String) -> Result<(), String> {
        match self {
            _ => Err(card),
        }
    }
}
