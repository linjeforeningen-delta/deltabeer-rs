use crate::app::command::Command;
use crate::app::fields::input::InputConstraint;
use crate::app::message::{DialogMessage, InputMessage};
use crate::app::{App, AppError, AuthenticationMessage, Dialog, DialogOpenMode, Message, TextInput, TransactionMessage, UserDialogState, UserMessage};

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
                let card = match self.dialogs.active_mut() {
                    Some(dialog) => {
                        match dialog.handle_scan(card) {
                            Ok(()) => {
                                self.status = "Card scanned".into();
                                return None;
                            }

                            Err(card) => card,
                        }
                    }

                    None => card,
                };

                self.status = "Looking up user...".into();
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

            Message::Authentication(message) => {
                self.update_authentication(message)
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
                self.dialogs.open(Dialog::User(UserDialogState {
                    user,
                    amount: TextInput::new(InputConstraint::Numeric),
                }),
                                  DialogOpenMode::Reset);

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
                self.dialogs.close();
                None
            }
        }
    }

    fn update_input(&mut self, message: InputMessage) -> Option<Command> {
        match message {
            InputMessage::Char(c) => {
                if let Some(dialog) = &mut self.dialogs.active_mut() {
                    if let Some(input) = dialog.input_mut() {
                        input.push(c);
                    }
                }

                None
            }

            InputMessage::Backspace => {
                if let Some(dialog) = &mut self.dialogs.active_mut() {
                    if let Some(input) = dialog.input_mut() {
                        input.backspace();
                    }
                }

                None
            }

            InputMessage::Submit => {
                match &mut self.dialogs.active_mut() {
                    Some(Dialog::User(state)) => {
                        if let Some(amount) = state.amount.as_u32() {
                            let user_id = state.user.id.clone();
                            self.dialogs.close();
                            Some(Command::Spend { user_id, amount })
                        } else {
                            self.status = "Invalid amount".into();
                            None
                        }
                    }

                    Some(Dialog::AdminAuth(state)) => {
                        let identifier = state.card.clone();
                        let password = state.password.as_str().to_string();
                        Some(Command::RequestAdminAuth { identifier: identifier?, password })
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

    fn update_authentication(&mut self, message: AuthenticationMessage) -> Option<Command> {
        match message {
            AuthenticationMessage::SingleUseToken(token) => {
                self.status = "Admin authentication successful".into();
                self.dialogs.close();
                None
            }

            AuthenticationMessage::AdminAuthFailed(error) => {
                self.status = format!("Admin authentication failed: {}", error);
                None
            }
        }
    }
}
