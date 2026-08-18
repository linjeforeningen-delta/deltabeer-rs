use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::message::ApiRequest;
use crate::app::{AppError, Message};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Default, Debug)]
pub(crate) struct RevokeAdminDialog {
    pub card: Option<String>,
}

impl RevokeAdminDialog {
    pub fn new() -> Self {
        Self { card: None }
    }
}

impl DialogBehavior for RevokeAdminDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Enter => {
                let identifier = self.card.clone();

                if identifier.is_none() {
                    return DialogResult::Message(Message::Failed(AppError::Validation(
                        "Card scan required for admin revocation".into(),
                    )));
                }

                DialogResult::Message(Message::ApiRequest(ApiRequest::RevokeAdmin {
                    identifier: identifier.unwrap(),
                }))
            }

            _ => DialogResult::Unhandled(key),
        }
    }

    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        self.card = Some(card);
        DialogResult::Consumed
    }
}
