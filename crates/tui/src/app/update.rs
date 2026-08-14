use crate::app::admin_action::AdminAction;
use crate::app::command::Command;
use crate::app::dialog::DialogResult;
use crate::app::message::Request;
use crate::app::{App, AppError, Message};

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
                            DialogResult::Consumed => {
                                return None;
                            }

                            DialogResult::Message(message) => {
                                return self.update(message);
                            }

                            DialogResult::Unhandled(card) => card,
                        }
                    }

                    None => card,
                };

                self.status = "Looking up user...".into();
                Some(Command::LookupUser(card))
            }

            Message::Failed(error) => {
                self.handle_error(error)
            }

            Message::Status(status) => {
                self.status = status;
                None
            }

            Message::Request(request) => {
                self.handle_request(request)
            }

            Message::DialogOpen { dialog, mode } => {
                self.dialogs.open(dialog, mode);
                None
            }

            Message::DialogClose => {
                self.dialogs.close();
                None
            }

            Message::AdminAuthenticated(token) => {
                self.status = "Admin authentication successful".into();
                self.complete_admin_auth(token)
            }
        }
    }

    fn handle_error(&mut self, error: AppError) -> Option<Command> {
        match error {
            AppError::Api(error) => {
                self.status = format!("API Error: {}", error);
                None
            }

            AppError::Validation(error) => {
                self.status = format!("Validation Error: {}", error);
                None
            }

            AppError::Authentication(error) => {
                self.status = format!("Authentication Error: {}", error);
                None
            }

            AppError::SessionExpired => {
                self.status = "Session expired".into();
                None
            }

            _ => None,
        }
    }

    fn handle_request(&mut self, request: Request) -> Option<Command> {
        match request {
            Request::LookupUser(card) => {
                self.status = "Looking up user...".into();
                Some(Command::LookupUser(card))
            }

            Request::Spend { user_id, amount } => {
                self.status = "Spending...".into();
                Some(Command::Spend { user_id, amount })
            }

            Request::TopUp { user_id, amount } => {
                self.status = "Topping up...".into();
                self.request_admin_action(AdminAction::TopUp { user_id, amount })
            }

            Request::AuthenticateAdmin { identifier, password } => {
                self.status = "Authenticating admin...".into();
                Some(Command::RequestAdminAuth { identifier, password })
            }
        }
    }
}
