use crate::app::admin_action::AdminAction;
use crate::app::command::Command;
use crate::app::dialog::{DialogResult, UserDialog};
use crate::app::message::{ApiRequest, ApiResult};
use crate::app::{App, AppError, DialogOpenMode, Message};

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Option<Command> {
        match message {
            Message::ApiRequest(request) => self.handle_request(request),

            Message::ApiResponse(result) => self.handle_request_result(result),

            Message::Status(status) => {
                self.status = status;
                None
            }

            Message::Failed(error) => self.handle_error(error),

            Message::OpenDialog { dialog, mode } => {
                self.dialogs.open(dialog, mode);
                None
            }

            Message::CloseDialog => {
                self.dialogs.close();
                None
            }

            Message::CardScanned(card) => self.handle_card_scan(card),

            Message::Navigate(page) => {
                self.page = page;
                None
            }

            Message::Quit => {
                self.should_quit = true;
                None
            }
        }
    }

    fn handle_request(&mut self, request: ApiRequest) -> Option<Command> {
        match request {
            ApiRequest::LookupUser(card) => {
                self.status = "Looking up user...".into();
                Some(Command::LookupUser(card))
            }

            ApiRequest::Spend { user_id, amount } => {
                self.status = "Spending...".into();
                Some(Command::Spend { user_id, amount })
            }

            ApiRequest::TopUp { user_id, amount } => {
                self.status = "Topping up...".into();
                self.request_admin_action(AdminAction::TopUp { user_id, amount })
            }

            ApiRequest::AuthenticateAdmin {
                identifier,
                password,
            } => {
                self.status = "Authenticating admin...".into();
                Some(Command::RequestAdminAuth {
                    identifier,
                    password,
                })
            }

            ApiRequest::MakeUser {
                name,
                username,
                program,
                card_number,
                birthdate,
            } => {
                self.status = "Creating user...".into();

                self.request_admin_action(AdminAction::MakeUser {
                    name,
                    username,
                    program,
                    card_number,
                    birthdate,
                })
            }

            ApiRequest::GrantAdmin {
                identifier,
                password,
            } => {
                self.status = "Granting admin...".into();
                self.request_admin_action(AdminAction::GrantAdmin {
                    identifier,
                    password,
                })
            }

            ApiRequest::RevokeAdmin { identifier } => {
                self.status = "Revoking admin...".into();
                self.request_admin_action(AdminAction::RevokeAdmin { identifier })
            }
        }
    }

    fn handle_request_result(&mut self, result: ApiResult) -> Option<Command> {
        match result {
            ApiResult::UserLoaded(user) => {
                self.status = format!("User {} loaded", user.name);
                self.dialogs
                    .open(Box::new(UserDialog::new(user)), DialogOpenMode::Reset);

                None
            }

            ApiResult::SpendSucceeded(transaction) => {
                self.status = format!("Spent {:?} successfully", transaction.amount);
                self.dialogs.close();
                None
            }

            ApiResult::TopUpSucceeded(transaction) => {
                self.status = format!("Topped up {:?} successfully", transaction.amount);
                Some(Command::LookupUser(transaction.user_id.to_string()))
            }

            ApiResult::AdminAuthenticated(token) => {
                self.status = "Admin authenticated".into();
                self.complete_admin_auth(token)
            }

            ApiResult::MakeUserSucceeded(user) => {
                self.status = format!("User {} created", user.name);
                self.dialogs.close();
                Some(Command::LookupUser(user.id.to_string()))
            }

            ApiResult::RoleChanged(user_id) => {
                self.status = "User role updated".into();
                Some(Command::LookupUser(user_id.to_string()))
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

    fn handle_card_scan(&mut self, card: String) -> Option<Command> {
        let card = match self.dialogs.active_mut() {
            Some(dialog) => match dialog.handle_scan(card) {
                DialogResult::Consumed => {
                    return None;
                }

                DialogResult::Message(message) => {
                    return self.update(message);
                }

                DialogResult::Unhandled(card) => card,
            },

            None => card,
        };

        self.status = "Looking up user...".into();

        Some(Command::LookupUser(card))
    }
}
