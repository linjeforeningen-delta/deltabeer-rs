use crate::api::{command::ApiCommand, request::ApiRequest, result::ApiResult};
use crate::app::dialog::menu::{set_locale, toggle_locale};
use crate::app::dialog::{DialogResult, UserDialog};
use crate::app::{App, AppError, DialogOpenMode, Message, StatusMessage};
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

            Message::OpenUser(user) => return self.open_user_dialog(user),

            Message::Navigate(page) => {
                self.page = page.into();
                if page == crate::app::PageId::Users {
                    return self.update(Message::ApiRequest(ApiRequest::ListUsers));
                }
            }

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
            ApiResult::Users(users) => {
                let count = users.len();
                if let crate::app::Page::Users(page) = &mut self.page {
                    page.set_users(users);
                }
                self.status = StatusMessage::UsersLoaded(count);
                None
            }
            ApiResult::LookupUser(user) => {
                self.status = StatusMessage::UserLoaded(user.name.clone());
                self.open_user_dialog(user)
            }

            ApiResult::Spend(transaction) => {
                self.status = StatusMessage::SpendSuccess(transaction.amount.0);
                self.dialogs.close();
                None
            }
            ApiResult::TopUp(transaction) => {
                self.status = StatusMessage::TopUpSuccess(transaction.amount.0);
                self.dialogs.close_to_admin_menu();
                self.request_api(ApiRequest::LookupUser(transaction.user_id.to_string()))
            }

            ApiResult::AuthenticateAdmin(_) => {
                tracing::error!("unexpected admin authentication response state");
                unreachable!("AuthenticateAdmin is handled before dialog routing")
            }

            ApiResult::StartAdminSession { user_id, token } => {
                tracing::info!(%user_id, "admin session started");
                self.auth = AuthState::Admin(AdminSession::new(user_id, token));
                self.status = StatusMessage::SessionStarted;
                self.dialogs.close();
                self.dialogs.set_auth_state(&self.auth);
                None
            }

            ApiResult::EndAdminSession => {
                tracing::info!("admin session ended");
                self.auth = AuthState::Normal;
                self.dialogs.set_auth_state(&self.auth);
                self.status = StatusMessage::SessionEnded;
                None
            }

            ApiResult::MakeUser(user) => {
                self.status = StatusMessage::UserCreated(user.name.clone());
                self.dialogs.close_to_admin_menu();
                self.request_api(ApiRequest::LookupUser(user.id.to_string()))
            }

            ApiResult::UpdateUser(user) => {
                self.status = StatusMessage::UserUpdated(user.name.clone());
                self.dialogs.close_to_admin_menu();
                self.request_api(ApiRequest::LookupUser(user.id.to_string()))
            }

            ApiResult::GrantAdmin(user_id) => {
                self.status = StatusMessage::AdminGranted(user_id.to_string());
                self.dialogs.close_to_admin_menu();
                None
            }

            ApiResult::RevokeAdmin(user_id) => {
                self.status = StatusMessage::AdminRevoked(user_id.to_string());
                self.dialogs.close_to_admin_menu();
                None
            }
        }
    }

    fn handle_error(&mut self, error: AppError) -> Option<ApiCommand> {
        tracing::warn!(error_code = error.code(), "application request failed");
        self.status = StatusMessage::Error(error);
        None
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

        self.status = StatusMessage::Progress(crate::app::ProgressMessage::LookingUp);

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

        let mode = if self.dialogs.is_admin_menu_active() {
            DialogOpenMode::Push
        } else {
            DialogOpenMode::Reset
        };
        self.dialogs.open(Box::new(UserDialog::new(user)), mode);

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
