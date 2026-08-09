use crate::app::Message;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn map_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}