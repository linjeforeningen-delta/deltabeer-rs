mod stack;
mod user;
mod admin_auth;
mod topup;
mod menu;

use crate::app::Message;
use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Debug;

use crate::ui::dialogs::DialogView;
pub(crate) use admin_auth::AdminAuthDialog;
pub(crate) use menu::dialog::MenuDialog;
pub(crate) use stack::{DialogOpenMode, DialogStack};
pub(crate) use topup::TopUpDialog;
pub(crate) use user::UserDialog;

pub(crate) enum DialogResult<T> {
    Consumed,
    Message(Message),
    Unhandled(T),
}

pub(crate) trait DialogBehavior: Debug {
    fn handle_key(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Esc => {
                DialogResult::Message(Message::CloseDialog)
            }

            _ => self.handle_key_inner(key),
        }
    }

    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent>;
    fn handle_scan(
        &mut self,
        card: String,
    ) -> DialogResult<String> {
        DialogResult::Unhandled(card)
    }
}


pub(crate) trait Dialog:
DialogBehavior + DialogView
{}

impl<T> Dialog for T
where
    T: DialogBehavior + DialogView,
{}

