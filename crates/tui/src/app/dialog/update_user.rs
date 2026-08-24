use crate::api::models::user::{User, UserPatch};
use crate::api::request::ApiRequest;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::fields::input::InputConstraint;
use crate::app::{AppError, Message, TextInput};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UpdateUserStage {
    Identify,
    Edit,
}

#[derive(Debug)]
pub(crate) struct UpdateUserDialog {
    pub(crate) stage: UpdateUserStage,

    pub(crate) identifier: TextInput,

    pub(crate) user: Option<User>,
    pub(crate) replacement_card: Option<String>,

    pub(crate) name: TextInput,
    pub(crate) username: TextInput,
    pub(crate) program: TextInput,
    pub(crate) comments: TextInput,

    pub(crate) active_field: usize,
}

impl UpdateUserDialog {
    pub(crate) fn new() -> Self {
        Self {
            stage: UpdateUserStage::Identify,
            identifier: TextInput::new(InputConstraint::Ascii),
            user: None,
            replacement_card: None,
            name: TextInput::new(InputConstraint::Any),
            username: TextInput::new(InputConstraint::Ascii),
            program: TextInput::new(InputConstraint::Any),
            comments: TextInput::new(InputConstraint::Any),
            active_field: 0,
        }
    }

    fn set_user(&mut self, user: User) {
        self.name.set_value(&user.name);
        self.username.set_value(&user.username);
        self.program.set_value(&user.program);
        self.comments.set_value(&user.comments);
        self.replacement_card = None;
        self.user = Some(user);
        self.stage = UpdateUserStage::Edit;
        self.active_field = 0;
    }

    fn active_input_mut(&mut self) -> &mut TextInput {
        match self.active_field {
            0 => &mut self.name,
            1 => &mut self.username,
            2 => &mut self.program,
            3 => &mut self.comments,
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
        let Some(user) = &self.user else {
            return DialogResult::Message(Message::Failed(AppError::Validation(
                t!("validation.user_not_identified").to_string(),
            )));
        };

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

        let comments = self.comments.as_str().trim();

        let card_opt = match &self.replacement_card {
            Some(card_str) => {
                let card_num = match card_str.trim().parse::<u32>() {
                    Ok(c) => c,
                    Err(_) => {
                        return DialogResult::Message(Message::Failed(AppError::Validation(
                            t!("validation.invalid_card").to_string(),
                        )));
                    }
                };
                if card_num != user.card_number {
                    Some(card_num)
                } else {
                    None
                }
            }
            None => None,
        };

        let name_opt = if name != user.name {
            Some(name.to_owned())
        } else {
            None
        };

        let username_opt = if username != user.username {
            Some(username.to_owned())
        } else {
            None
        };

        let program_opt = if program != user.program {
            Some(program.to_owned())
        } else {
            None
        };

        let comments_opt = if comments != user.comments {
            Some(comments.to_owned())
        } else {
            None
        };

        if name_opt.is_none()
            && username_opt.is_none()
            && program_opt.is_none()
            && card_opt.is_none()
            && comments_opt.is_none()
        {
            return DialogResult::Message(Message::Status(t!("status.no_changes").to_string()));
        }

        let patch = UserPatch {
            name: name_opt,
            username: username_opt,
            program: program_opt,
            card_number: card_opt,
            comments: comments_opt,
        };

        DialogResult::Message(Message::ApiRequest(ApiRequest::UpdateUser {
            user_id: user.id,
            patch,
        }))
    }
}

impl DialogBehavior for UpdateUserDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match self.stage {
            UpdateUserStage::Identify => match key.code {
                KeyCode::Char(c) => {
                    self.identifier.push(c);
                    DialogResult::Consumed
                }
                KeyCode::Backspace => {
                    self.identifier.backspace();
                    DialogResult::Consumed
                }
                KeyCode::Enter => {
                    let identifier = self.identifier.as_str().trim();
                    if identifier.is_empty() {
                        DialogResult::Consumed
                    } else {
                        DialogResult::Message(Message::ApiRequest(ApiRequest::LookupUser(
                            identifier.to_owned(),
                        )))
                    }
                }
                _ => DialogResult::Unhandled(key),
            },
            UpdateUserStage::Edit => match key.code {
                KeyCode::Down | KeyCode::Tab => {
                    self.next_field();
                    DialogResult::Consumed
                }
                KeyCode::Up | KeyCode::BackTab => {
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
            },
        }
    }

    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        match self.stage {
            UpdateUserStage::Identify => DialogResult::Unhandled(card),
            UpdateUserStage::Edit => {
                self.replacement_card = Some(card);
                DialogResult::Consumed
            }
        }
    }

    fn handle_api_result(
        &mut self,
        result: crate::api::result::ApiResult,
    ) -> DialogResult<crate::api::result::ApiResult> {
        match (&self.stage, result) {
            (UpdateUserStage::Identify, crate::api::result::ApiResult::LookupUser(user)) => {
                self.set_user(user);
                DialogResult::Consumed
            }
            (_, result) => DialogResult::Unhandled(result),
        }
    }
}
