use crate::api::models::user::User;
use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::{AppError, Message, TextInput};

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct GrantAdminDialog {
    pub(crate) user: Option<User>,

    pub(crate) password: TextInput,
    pub(crate) confirm_password: TextInput,

    pub(crate) active_field: usize,
}

impl GrantAdminDialog {
    pub(crate) fn new() -> Self {
        Self {
            user: None,

            password: TextInput::new(InputConstraint::Any),
            confirm_password: TextInput::new(InputConstraint::Any),

            active_field: 0,
        }
    }

    fn active_input_mut(&mut self) -> &mut TextInput {
        match self.active_field {
            0 => &mut self.password,
            1 => &mut self.confirm_password,

            _ => unreachable!(),
        }
    }

    fn next_field(&mut self) {
        self.active_field = (self.active_field + 1) % 2;
    }

    fn previous_field(&mut self) {
        self.active_field = self.active_field.checked_sub(1).unwrap_or(1);
    }

    fn submit(&self) -> DialogResult<KeyEvent> {
        let Some(user) = &self.user else {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.card_required_grant").to_string(),
            )));
        };

        let password = self.password.as_str();

        if password.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.password_required").to_string(),
            )));
        }

        let confirm_password = self.confirm_password.as_str();

        if confirm_password.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.confirm_required").to_string(),
            )));
        }

        if password != confirm_password {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.passwords_mismatch").to_string(),
            )));
        }

        DialogResult::Message(Message::ApiRequest(ApiRequest::GrantAdmin {
            user_id: user.id,
            password: password.to_owned(),
        }))
    }
}

impl DialogBehavior for GrantAdminDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Down => {
                self.next_field();
                DialogResult::Consumed
            }

            KeyCode::Up => {
                self.previous_field();
                DialogResult::Consumed
            }

            KeyCode::Tab => {
                self.next_field();
                DialogResult::Consumed
            }

            KeyCode::BackTab => {
                self.previous_field();
                DialogResult::Consumed
            }

            KeyCode::Char(c) => {
                self.active_input_mut().push(c);
                DialogResult::Consumed
            }

            KeyCode::Backspace => {
                self.active_input_mut().backspace();
                DialogResult::Consumed
            }

            KeyCode::Enter => self.submit(),

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
