use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::message::Request;
use crate::app::{AppError, Message, TextInput};

use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct GrantAdminDialog {
    pub(crate) card: Option<String>,

    pub(crate) password: TextInput,
    pub(crate) confirm_password: TextInput,

    pub(crate) active_field: usize,
}

impl GrantAdminDialog {
    pub(crate) fn new() -> Self {
        Self {
            card: None,

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
        let identifier = self.card.clone();

        if identifier.is_none() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                "Card scan required for admin revocation".into(),
            )));
        }

        let password = self.password.as_str();

        if password.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                "Password is required".into(),
            )));
        }

        let confirm_password = self.confirm_password.as_str();

        if confirm_password.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                "Password confirmation is required".into(),
            )));
        }

        if password != confirm_password {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                "Passwords do not match".into(),
            )));
        }

        DialogResult::Message(Message::Request(Request::GrantAdmin {
            identifier: self.card.clone().unwrap(),
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

    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        self.card = Some(card);
        DialogResult::Consumed
    }
}
