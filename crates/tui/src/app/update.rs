use crate::api::{command::ApiCommand, request::ApiRequest, result::ApiResult};
use crate::app::dialog::menu::{set_locale, toggle_locale};
use crate::app::dialog::{DialogResult, UserDialog};
use crate::app::{App, AppError, DialogOpenMode, Message};
use crate::auth::{AdminContext, AdminSession, AuthState};
use crate::model::{Role, User};

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Option<ApiCommand> {
        match message {
            Message::ApiRequest(request) => return self.handle_api_request(request),

            Message::ApiResponse(result) => return self.handle_api_result(result),

            Message::Status(status) => self.status = status,

            Message::Failed(error) => return self.handle_error(error),

            Message::OpenDialog { dialog, mode } => self.dialogs.open(dialog, mode),

            Message::OpenAdminDialog { dialog, mode } => {
                self.dialogs
                    .open_admin(dialog, mode, &self.auth, self.active_admin.clone())
            }

            Message::CloseDialog => return self.handle_close_dialog(),

            Message::CardScanned(card) => return self.handle_card_scan(card),

            Message::Navigate(page) => self.page = page.into(),

            Message::Quit => self.should_quit = true,

            Message::SetLanguage(language) => {
                set_locale(language);
                self.dialogs.close();
            }

            Message::ToggleLanguage => toggle_locale(),
        };
        None
    }

    fn handle_api_request(&mut self, request: ApiRequest) -> Option<ApiCommand> {
        self.status = request.status_message();

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
                self.status = t!("status.user_loaded", name = user.name).to_string();
                self.open_user_dialog(user)
            }

            ApiResult::Spend(transaction) => {
                self.status = t!("status.spend_success", amount = transaction.amount.0).to_string();
                self.dialogs.close();
                None
            }
            ApiResult::TopUp(transaction) => {
                self.status = t!("status.topup_success", amount = transaction.amount.0).to_string();
                self.dialogs.close();
                self.request_api(ApiRequest::LookupUser(transaction.user_id.to_string()))
            }

            ApiResult::AuthenticateAdmin(_) => {
                unreachable!("AuthenticateAdmin is handled before dialog routing")
            }

            ApiResult::StartAdminSession { user_id, token } => {
                self.auth = AuthState::Admin(AdminSession::new(user_id, token));
                self.status = t!("status.session_started").to_string();
                self.dialogs.close();
                self.dialogs.set_auth_state(&self.auth);
                None
            }

            ApiResult::EndAdminSession => {
                self.auth = AuthState::Normal;
                self.dialogs.set_auth_state(&self.auth);
                self.status = t!("status.session_ended").to_string();
                None
            }

            ApiResult::MakeUser(user) => {
                self.status = t!("status.user_created", name = user.name).to_string();
                self.dialogs.close();
                self.request_api(ApiRequest::LookupUser(user.id.to_string()))
            }

            ApiResult::UpdateUser(user) => {
                self.status = t!("status.user_updated", name = user.name).to_string();
                self.dialogs.close();
                self.request_api(ApiRequest::LookupUser(user.id.to_string()))
            }

            ApiResult::GrantAdmin(user_id) => {
                self.status = t!("status.admin_granted", id = user_id).to_string();
                self.dialogs.close();
                None
            }

            ApiResult::RevokeAdmin(user_id) => {
                self.status = t!("status.admin_revoked", id = user_id).to_string();
                self.dialogs.close();
                None
            }
        }
    }

    fn handle_error(&mut self, error: AppError) -> Option<ApiCommand> {
        match error {
            AppError::Api => {
                self.status = t!("errors.api").to_string();
                None
            }

            AppError::Unauthorized => {
                self.status = t!("errors.unauthorized").to_string();
                None
            }

            AppError::Forbidden => {
                self.status = t!("errors.forbidden").to_string();
                None
            }

            AppError::NotFound => {
                self.status = t!("errors.not_found").to_string();
                None
            }

            AppError::InvalidUserIdentifier => {
                self.status = t!("errors.invalid_user_identifier").to_string();
                None
            }

            AppError::Conflict => {
                self.status = t!("errors.conflict").to_string();
                None
            }

            AppError::BadRequest => {
                self.status = t!("errors.bad_request").to_string();
                None
            }

            AppError::Transport => {
                self.status = t!("errors.network").to_string();
                None
            }

            AppError::InvalidResponse => {
                self.status = t!("errors.invalid_response").to_string();
                None
            }

            AppError::Validation(error) => {
                self.status = format!("{}: {}", t!("errors.validation"), error);
                None
            }

            AppError::Authentication(error) => {
                self.status = format!("{}: {}", t!("errors.authentication"), error);
                None
            }

            AppError::SessionExpired => {
                self.status = t!("status.session_expired").to_string();
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

        self.status = t!("progress.looking_up").to_string();

        self.request_api(ApiRequest::LookupUser(card))
    }

    fn open_user_dialog(&mut self, user: User) -> Option<ApiCommand> {
        let previous_session_needs_logout = match (&self.auth, &user.role) {
            (AuthState::Admin(session), Role::Admin) => session.user_id != user.id,
            (AuthState::Admin(_), _) => true,
            _ => false,
        };

        self.active_admin = match user.role {
            Role::Admin => Some(AdminContext {
                user_id: user.id,
                name: user.name.clone(),
            }),
            _ => None,
        };

        self.dialogs
            .open(Box::new(UserDialog::new(user)), DialogOpenMode::Reset);

        if previous_session_needs_logout {
            self.request_api(ApiRequest::EndAdminSession)
        } else {
            None
        }
    }

    fn handle_close_dialog(&mut self) -> Option<ApiCommand> {
        self.dialogs.close();

        if !self.dialogs.is_empty() {
            return None;
        }

        self.active_admin = None;
        self.request_api(ApiRequest::EndAdminSession)
    }
}
