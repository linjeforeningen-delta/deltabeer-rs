use crate::api::models::user::User;
use crate::api::request::ApiRequest;
use crate::api::result::ApiResult;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::{AppError, Message};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default, Debug)]
pub(crate) struct RevokeAdminDialog {
    pub(crate) user: Option<User>,
}

impl RevokeAdminDialog {
    pub fn new() -> Self {
        Self { user: None }
    }
}

impl DialogBehavior for RevokeAdminDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Enter => {
                let Some(user) = &self.user else {
                    return DialogResult::Message(Message::Failed(AppError::Validation(
                        "Card scan required for admin revocation".into(),
                    )));
                };

                DialogResult::Message(Message::ApiRequest(ApiRequest::RevokeAdmin {
                    user_id: user.id,
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }

    fn handle_api_result(&mut self, result: ApiResult) -> DialogResult<ApiResult> {
        match result {
            ApiResult::LookupUser(user) => {
                self.user = Some(user);
                DialogResult::Consumed
            }
            _ => DialogResult::Unhandled(result),
        }
    }
}
