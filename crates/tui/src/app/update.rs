use crate::api::command::ApiCommand;
use crate::api::models::user::{Role, User};
use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::dialog::{DialogResult, UserDialog};
use crate::app::{App, AppError, DialogOpenMode, Message};
use crate::auth::{AdminContext, AuthState};

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Option<ApiCommand> {
        match message {
            Message::ApiRequest(request) => return self.handle_api_request(request),

            Message::ApiResponse(result) => return self.handle_api_result(result),

            Message::Status(status) => self.status = status,

            Message::Failed(error) => return self.handle_error(error),

            Message::OpenDialog { dialog, mode } => self.dialogs.open(dialog, mode),

            Message::CloseDialog => self.handle_close_dialog(),

            Message::CardScanned(card) => return self.handle_card_scan(card),

            Message::Navigate(page) => self.page = page,

            Message::Quit => self.should_quit = true,
        };
        None
    }

    fn handle_api_request(
        &mut self,
        request: ApiRequest,
    ) -> Option<ApiCommand> {
        self.status = request.status_message().into();

        self.request_api(request)
    }

    fn handle_api_result(&mut self, result: ApiResult) -> Option<ApiCommand> {
        if let ApiResult::AuthenticateAdmin(token) = result {
            return self.complete_pending_request(token);
        }

        let result = match self.dialogs.active_mut() {
            Some(dialog) => match dialog.handle_api_result(result) {
                DialogResult::Consumed => return None,

                DialogResult::Message(message) => {
                    return self.update(message);
                }

                DialogResult::Unhandled(result) => result,
            },

            None => result,
        };

        self.handle_api_result_default(result)
    }

    fn handle_api_result_default(&mut self, result: ApiResult) -> Option<ApiCommand> {
        match result {
            ApiResult::LookupUser(user) => {
                self.status = format!("User {} loaded", user.name);
                self.open_user_dialog(user);
                None
            }

            ApiResult::Spend(transaction) => {
                self.status = format!("Spent {:?} successfully", transaction.amount);
                self.dialogs.close();
                None
            }
            ApiResult::TopUp(transaction) => {
                self.status = format!("Topped up {:?} successfully", transaction.amount);
                self.request_api(
                    ApiRequest::LookupUser(
                        transaction.user_id.to_string()
                    )
                )
            }

            ApiResult::AuthenticateAdmin(_) => {
                unreachable!("AuthenticateAdmin is handled before dialog routing")
            }

            ApiResult::MakeUser(user) => {
                self.status = format!("User {} created", user.name);
                self.dialogs.close();
                self.request_api(
                    ApiRequest::LookupUser(
                        user.id.to_string()
                    )
                )
            }

            ApiResult::GrantAdmin(user_id) => {
                self.status = format!("Granted admin to user {}", user_id);
                self.dialogs.close();
                None
            }

            ApiResult::RevokeAdmin(user_id) => {
                self.status = format!("Revoked admin from user {}", user_id);
                self.dialogs.close();
                None
            }
        }
    }

    fn handle_error(&mut self, error: AppError) -> Option<ApiCommand> {
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
        }
    }

    fn handle_card_scan(&mut self, card: String) -> Option<ApiCommand> {
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

        self.request_api(ApiRequest::LookupUser(card))
    }

    fn open_user_dialog(
        &mut self,
        user: User,
    ) {
        if user.role == Role::Admin {
            self.active_admin = Some(AdminContext {
                user_id: user.id.clone(),
                name: user.name.clone(),
            });
        }
        self.dialogs.open(Box::new(UserDialog::new(user)), DialogOpenMode::Reset)
    }

    fn handle_close_dialog(&mut self) {
        if self.dialogs.is_empty() {
            self.active_admin = None;
            self.auth = AuthState::Normal;
        }
        self.dialogs.close();
    }
}
