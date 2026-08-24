use crate::api::request::ApiRequest;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::{AppError, Message, TextInput};
use chrono::NaiveDate;
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub(crate) struct MakeUserDialog {
    pub(crate) name: TextInput,
    pub(crate) username: TextInput,
    pub(crate) program: TextInput,
    pub(crate) birthdate: TextInput,

    pub(crate) card: Option<String>,

    pub(crate) active_field: usize,
}

impl MakeUserDialog {
    pub(crate) fn new() -> Self {
        Self {
            name: TextInput::new(InputConstraint::Any),
            username: TextInput::new(InputConstraint::Ascii),
            program: TextInput::new(InputConstraint::Any),
            birthdate: TextInput::new(InputConstraint::Ascii),

            card: None,

            active_field: 0,
        }
    }

    fn active_input_mut(&mut self) -> &mut TextInput {
        match self.active_field {
            0 => &mut self.name,
            1 => &mut self.username,
            2 => &mut self.program,
            3 => &mut self.birthdate,

            _ => unreachable!(),
        }
    }

    fn next_field(&mut self) {
        self.active_field = (self.active_field + 1) % 4;
    }

    fn previous_field(&mut self) {
        self.active_field = self.active_field.checked_sub(1).unwrap_or(3);
    }

    fn submit(&self) -> DialogResult<KeyEvent> {
        let name = self.name.as_str().trim();

        if name.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.name_required").to_string(),
            )));
        }

        let username = self.username.as_str().trim();

        if username.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.username_required").to_string(),
            )));
        }

        let program = self.program.as_str().trim();

        if program.is_empty() {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.program_required").to_string(),
            )));
        }

        let birthdate = match NaiveDate::parse_from_str(self.birthdate.as_str().trim(), "%Y-%m-%d")
        {
            Ok(date) => date,

            Err(_) => {
                return DialogResult::Message(Message::Failed(AppError::Validation(
                    t!("validation.birthdate_format").to_string(),
                )));
            }
        };

        let Some(card) = &self.card else {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.scan_card_first").to_string(),
            )));
        };

        let card_number = match card.parse::<u32>() {
            Ok(card) => card,

            Err(_) => {
                return DialogResult::Message(Message::Failed(AppError::Validation(
                    t!("validation.invalid_card").to_string(),
                )));
            }
        };

        DialogResult::Message(Message::ApiRequest(ApiRequest::MakeUser {
            name: name.to_owned(),
            username: username.to_owned(),
            program: program.to_owned(),
            card_number,
            birthdate,
        }))
    }
}

impl DialogBehavior for MakeUserDialog {
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
