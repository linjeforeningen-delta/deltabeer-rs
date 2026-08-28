use crate::api::request::ApiRequest;
use crate::app::{AppError, DialogOpenMode, Message, TextInput, ValidationMessage};
use crate::app::{
    dialog::{DialogBehavior, DialogResult, menu::preset::admin},
    fields::input::InputConstraint,
};
use crate::model::{Role, User};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct UserDialog {
    pub(crate) user: User,
    pub(crate) amount: TextInput,
}

impl UserDialog {
    pub(crate) fn new(user: User) -> Self {
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
                    return DialogResult::Message(Message::Failed(AppError::Validation(
                        ValidationMessage::InvalidAmount,
                    )));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::Spend {
                    user_id: self.user.id,
                    amount,
                }))
            }

            KeyCode::Char('a') => {
                if self.user.role == Role::Admin {
                    return DialogResult::Message(Message::OpenAdminDialog {
                        dialog: Box::new(admin::new()),
                        mode: DialogOpenMode::Push,
                    });
                }
                DialogResult::Unhandled(key)
            }
            _ => DialogResult::Unhandled(key),
        }
    }
}
