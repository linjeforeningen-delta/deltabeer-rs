use crate::api::models::user::User;
use crate::app::dialog::topup::TopUpDialog;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::message::Request;
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
                    return DialogResult::Message(
                        Message::Status(
                            "Invalid amount".into()
                        )
                    );
                };

                DialogResult::Message(
                    Message::Request(
                        Request::Spend {
                            user_id: self.user.id.clone(),
                            amount,
                        }
                    )
                )
            }

            KeyCode::Char('t') => {
                DialogResult::Message(
                    Message::DialogOpen {
                        dialog: Box::new(TopUpDialog {
                            user: self.user.clone(),
                            amount: TextInput::new(InputConstraint::Numeric),
                        }),
                        mode: DialogOpenMode::Push,
                    }
                )
            }
            _ => DialogResult::Unhandled(key),
        }
    }

    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        DialogResult::Unhandled(card)
    }
}

