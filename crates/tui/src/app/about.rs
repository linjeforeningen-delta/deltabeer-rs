use crossterm::event::{KeyCode, KeyEvent};

use crate::app::Message;
use crate::app::dialog::{DialogBehavior, DialogResult};

#[derive(Debug, Default)]
pub(crate) struct AboutDialog;

impl AboutDialog {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl DialogBehavior for AboutDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        match key.code {
            KeyCode::Enter => DialogResult::Message(Message::CloseDialog),
            _ => DialogResult::Unhandled(key),
        }
    }
}
