use crate::api::request::ApiRequest;
use crate::app::{AppError, Message, TextInput};
use crate::app::{
    dialog::{DialogBehavior, DialogResult},
    fields::input::InputConstraint,
};
use crate::auth::AdminContext;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct AdminAuthDialog {
    pub(crate) admin: Option<AdminContext>,
    pub password: TextInput,
}

impl AdminAuthDialog {
    pub fn new(admin: Option<AdminContext>) -> Self {
        Self {
            admin,
            password: TextInput::new(InputConstraint::Any),
        }
    }

    pub fn empty() -> Self {
        Self::new(None)
    }

    pub fn with_admin(admin: AdminContext) -> Self {
        Self::new(Some(admin))
    }
}

impl DialogBehavior for AdminAuthDialog {
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

                let admin = match &self.admin {
                    Some(admin) => admin,
                    None => {
                        return DialogResult::Message(Message::Failed(AppError::Validation(
                            t!("validation.admin_required_auth").to_string(),
                        )));
                    }
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::AuthenticateAdmin {
                    user_id: admin.user_id,
                    password,
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }
}
