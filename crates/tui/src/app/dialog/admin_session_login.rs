use crate::api::request::ApiRequest;
use crate::app::{Message, TextInput};
use crate::app::{
    dialog::{AdminDialog, DialogBehavior, DialogResult},
    fields::input::InputConstraint,
};
use crate::auth::AdminContext;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct AdminSessionLoginDialog {
    pub(crate) admin: Option<AdminContext>,
    pub password: TextInput,
}

impl AdminSessionLoginDialog {
    pub(crate) fn new() -> Self {
        Self {
            admin: None,
            password: TextInput::new(InputConstraint::Any),
        }
    }
}

impl DialogBehavior for AdminSessionLoginDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Char(c) => {
                self.password.push(c);
                DialogResult::Consumed
            }

            KeyCode::Backspace => {
                self.password.backspace();
                DialogResult::Consumed
            }

            KeyCode::Enter => {
                let password = self.password.as_str().to_string();

                let Some(admin) = self.admin.as_ref() else {
                    return DialogResult::Message(Message::CloseDialog);
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::StartAdminSession {
                    user_id: admin.user_id,
                    password,
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }
}

impl AdminDialog for AdminSessionLoginDialog {
    fn set_admin_context(&mut self, context: Option<AdminContext>) {
        self.admin = context;
    }
}
