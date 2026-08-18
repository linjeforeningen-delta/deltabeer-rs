use crate::api::models::user::{Role, User};
use crate::api::request::ApiRequest;
use crate::app::dialog::menu::presets::admin_menu;
use crate::app::dialog::topup::TopUpDialog;
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
                    return DialogResult::Message(Message::Status("Invalid amount".into()));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::Spend {
                    user_id: self.user.id.clone(),
                    amount,
                }))
            }

            KeyCode::Char('t') => DialogResult::Message(Message::OpenDialog {
                dialog: Box::new(TopUpDialog {
                    user: self.user.clone(),
                    amount: TextInput::new(InputConstraint::Numeric),
                }),
                mode: DialogOpenMode::Push,
            }),

            KeyCode::Char('a') => {
                if self.user.role == Role::Admin {
                    return DialogResult::Message(Message::OpenDialog {
                        dialog: Box::new(admin_menu(&self.user.id)),
                        mode: DialogOpenMode::Push,
                    });
                }
                DialogResult::Unhandled(key)
            }
            _ => DialogResult::Unhandled(key),
        }
    }
}
