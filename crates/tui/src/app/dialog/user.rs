use crate::api::models::user::{Role, User};
use crate::api::request::ApiRequest;
use crate::app::dialog::menu::presets::admin_menu;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::{DialogOpenMode, Message, TextInput};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct UserDialog {
    pub user: User,
    pub amount: TextInput,
}

impl UserDialog {
    pub fn new(user: User) -> Self {
        Self {
            user,
            amount: TextInput::new(InputConstraint::Numeric),
        }
    }
}

impl DialogBehavior for UserDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.amount.push(c);
                DialogResult::Consumed
            }

            KeyCode::Backspace => {
                self.amount.backspace();
                DialogResult::Consumed
            }

            KeyCode::Enter => {
                let Some(amount) = self.amount.as_u32() else {
                    return DialogResult::Message(Message::Status(
                        t!("validation.invalid_amount").to_string(),
                    ));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::Spend {
                    user_id: self.user.id.clone(),
                    amount,
                }))
            }

            KeyCode::Char('a') => {
                if self.user.role == Role::Admin {
                    return DialogResult::Message(Message::OpenAdminDialog {
                        dialog: Box::new(admin_menu()),
                        mode: DialogOpenMode::Push,
                    });
                }
                DialogResult::Unhandled(key)
            }
            _ => DialogResult::Unhandled(key),
        }
    }
}
