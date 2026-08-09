use crate::app::{App, Message};
use crossterm::event::{KeyCode, KeyEvent};

pub(crate) fn map_key(
    app: &App,
    key: KeyEvent,
) -> Option<Message> {
    if app.dialog.is_some() {
        return map_key_dialog(app, key);
    }

    if let Some(message) = map_key_base(app, key) {
        return Some(message);
    }

    match app.page {
        crate::app::Page::Home => map_key_home(app, key),
        crate::app::Page::Users => map_key_users(app, key),
        crate::app::Page::Transactions => map_key_transactions(app, key),
        crate::app::Page::Stats => map_key_stats(app, key),
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

fn map_key_home(
    _app: &App,
    _key: KeyEvent,
) -> Option<Message> {
    todo!()
}

fn map_key_users(
    _app: &App,
    key: KeyEvent,
) -> Option<Message> {
    todo!()
}

fn map_key_transactions(
    _app: &App,
    _key: KeyEvent,
) -> Option<Message> {
    todo!()
}

fn map_key_stats(
    _app: &App,
    _key: KeyEvent,
) -> Option<Message> {
    todo!()
}