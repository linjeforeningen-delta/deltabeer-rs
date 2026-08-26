use crate::api::{request::ApiRequest, result::ApiResult};
use crate::app::{AppError, Message, TextInput};
use crate::app::{
    dialog::{DialogBehavior, DialogResult},
    fields::input::InputConstraint,
};
use crate::model::User;
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
                    return DialogResult::Message(Message::Status(
                        t!("validation.invalid_amount").to_string(),
                    ));
                };

                let Some(user) = &self.user else {
                    return DialogResult::Message(Message::Failed(AppError::Validation(
                        t!("validation.card_required_topup").to_string(),
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
