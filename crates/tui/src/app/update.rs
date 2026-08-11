use crate::app::command::Command;
use crate::app::message::{DialogMessage, InputMessage};
use crate::app::state::{Dialog, UserDialogState};
use crate::app::{App, AppError, Message, NumericInput, TransactionMessage, UserMessage};

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Option<Command> {
        match message {
            Message::Quit => {
                self.should_quit = true;
                None
            }

            Message::Navigate(page) => {
                self.page = page;
                None
            }

            Message::CardScanned(card) => {
                Some(Command::LookupUser(card))
            }

            Message::Failed(error) => {
                self.update_error(error)
            }

            Message::User(message) => {
                self.update_user(message)
            }

            Message::Dialog(message) => {
                self.update_dialog(message)
            }

            Message::Input(message) => {
                self.update_input(message)
            }

            Message::Transaction(message) => {
                self.update_transaction(message)
            }
        }
    }

    fn update_error(&mut self, error: AppError) -> Option<Command> {
        match error {
            crate::app::AppError::Api(error) => {
                self.status = format!("API Error: {}", error);
                None
            }

            crate::app::AppError::Validation(error) => {
                self.status = format!("Validation Error: {}", error);
                None
            }

            crate::app::AppError::Authentication(error) => {
                self.status = format!("Authentication Error: {}", error);
                None
            }

            crate::app::AppError::SessionExpired => {
                self.status = "Session expired".into();
                None
            }

            _ => None,
        }
    }

    fn update_user(&mut self, message: UserMessage) -> Option<Command> {
        match message {
            UserMessage::Loaded(user) => {
                self.dialog = Some(Dialog::User(UserDialogState {
                    user,
                    amount: NumericInput::new(),
                }));

                self.status = "User loaded".into();
                None
            }

            UserMessage::LoadFailed(error) => {
                self.status = error;
                None
            }
        }
    }

    fn update_dialog(&mut self, message: DialogMessage) -> Option<Command> {
        match message {
            DialogMessage::Close => {
                self.dialog = None;
                None
            }
        }
    }

    fn update_input(&mut self, message: InputMessage) -> Option<Command> {
        match message {
            InputMessage::Numeric(c) => {
                if let Some(dialog) = &mut self.dialog {
                    if let Some(input) = dialog.numeric_input_mut() {
                        input.push(c);
                    }
                }

                None
            }

            InputMessage::Backspace => {
                if let Some(dialog) = &mut self.dialog {
                    if let Some(input) = dialog.numeric_input_mut() {
                        input.backspace();
                    }
                }

                None
            }

            InputMessage::Submit => {
                match &mut self.dialog {
                    Some(Dialog::User(state)) => {
                        if let Some(amount) = state.amount.value() {
                            let user_id = state.user.id.clone();
                            self.dialog = None;
                            Some(Command::Spend { user_id, amount })
                        } else {
                            self.status = "Invalid amount".into();
                            None
                        }
                    }

                    _ => None,
                }
            }
        }
    }

    fn update_transaction(&mut self, message: TransactionMessage) -> Option<Command> {
        match message {
            TransactionMessage::SpendSuccess(transaction) => {
                self.status = "Spend successful".into();
                None
            }

            TransactionMessage::SpendFailed(error) => {
                self.status = error;
                None
            }
        }
    }
}