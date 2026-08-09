use crate::app::{App, Message};
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn map_key(
    app: &App,
    key: KeyEvent,
) -> Option<Message> {
    if app.dialog.is_some() {
        map_key_dialog(app, key)
    } else {
        map_key_base(app, key)
    }
}

fn map_key_dialog(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::CloseDialog),

        _ => None,
    }
}

fn map_key_base(app: &App, key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::Quit),

        KeyCode::Char('1') => Some(Message::OpenHome),
        KeyCode::Char('2') => Some(Message::OpenUsers),
        KeyCode::Char('3') => Some(Message::OpenTransactions),
        KeyCode::Char('4') => Some(Message::OpenStats),

        _ => None,
    }
}