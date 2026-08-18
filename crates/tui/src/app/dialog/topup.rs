use crate::api::models::user::User;
use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::{AppError, Message, TextInput};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct TopUpDialog {
    pub(crate) user: Option<User>,
    pub amount: TextInput,
}

impl TopUpDialog {
    pub fn new() -> Self {
        Self {
            user: None,
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

                let Some(user) = &self.user else {
                    return DialogResult::Message(Message::Failed(AppError::Validation(
                        "Card scan required for top-up".into(),
                    )));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::TopUp {
                    user_id: user.id,
                    amount,
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }

    fn handle_api_result(&mut self, result: ApiResult) -> DialogResult<ApiResult> {
        match result {
            ApiResult::LookupUser(user) => {
                self.user = Some(user);
                DialogResult::Consumed
            }
            _ => DialogResult::Unhandled(result),
        }
    }
}