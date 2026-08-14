use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::message::Request;
use crate::app::{AppError, Message, TextInput};
use crossterm::event::{KeyCode, KeyEvent};


#[derive(Default, Debug)]
pub(crate) struct AdminAuthDialog {
    pub card: Option<String>,
    pub password: TextInput,
}


impl DialogBehavior for AdminAuthDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Char(c) => {
                self.password.push(c);
                DialogResult::Consumed
            }

            KeyCode::Backspace => {
                self.password.backspace();
                DialogResult::Consumed
            }

            KeyCode::Enter => {
                let identifier = self.card.clone();
                let password = self.password.as_str().to_string();

                if identifier.is_none() {
                    return DialogResult::Message(
                        Message::Failed(AppError::Validation("Card scan required for admin authentication".into()))
                    );
                }

                DialogResult::Message(
                    Message::Request(
                        Request::AuthenticateAdmin {
                            identifier: identifier.unwrap(),
                            password,
                        }
                    )
                )
            }

            _ => DialogResult::Unhandled(key),
        }
    }

    fn handle_scan(&mut self, card: String) -> DialogResult<String> {
        self.card = Some(card);
        DialogResult::Consumed
    }
}
