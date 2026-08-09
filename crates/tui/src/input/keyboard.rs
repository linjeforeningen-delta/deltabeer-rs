use crate::app::Message;
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn map_key(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::Quit),
        
        KeyCode::Char('1') => Some(Message::OpenHome),
        KeyCode::Char('2') => Some(Message::OpenUsers),
        KeyCode::Char('3') => Some(Message::OpenTransactions),
        KeyCode::Char('4') => Some(Message::OpenStats),

        _ => None,
    }
}