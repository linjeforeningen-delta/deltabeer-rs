use crate::api::models::user::User;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::message::ApiRequest;
use crate::app::{Message, TextInput};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]

pub(crate) struct TopUpDialog {
    pub user: User,
    pub amount: TextInput,
}

impl TopUpDialog {
    pub fn new(user: User) -> Self {
        Self {
            user,
            amount: TextInput::new(InputConstraint::Numeric),
        }
    }
}

impl DialogBehavior for TopUpDialog {
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
                    return DialogResult::Message(Message::Status("Invalid amount".into()));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::TopUp {
                    user_id: self.user.id.clone(),
                    amount,
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }
}
