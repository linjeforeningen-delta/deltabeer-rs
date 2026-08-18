mod admin_auth;
mod grant_admin;
mod make_user;
mod menu;
mod revoke_admin;
mod stack;
mod topup;
mod user;

use crate::app::Message;
use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Debug;

use crate::api::result::ApiResult;
use crate::ui::dialogs::DialogView;
pub(crate) use admin_auth::AdminAuthDialog;
pub(crate) use grant_admin::GrantAdminDialog;
pub(crate) use make_user::MakeUserDialog;
pub(crate) use menu::dialog::MenuDialog;
pub(crate) use revoke_admin::RevokeAdminDialog;
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
            KeyCode::Esc => DialogResult::Message(Message::CloseDialog),

            _ => self.handle_key_inner(key),
        }
    }

    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent>;
    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        DialogResult::Unhandled(card)
    }

    fn handle_api_result(&mut self, result: ApiResult) -> DialogResult<ApiResult> {
        DialogResult::Unhandled(result)
    }
}

pub(crate) trait Dialog: DialogBehavior + DialogView {}

impl<T> Dialog for T where T: DialogBehavior + DialogView {}
