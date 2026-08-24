use crate::app::dialog::DialogResult;
use crate::app::dialog::menu::preset;
use crate::app::{App, Message, Page};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn map_key(app: &mut App, key: KeyEvent) -> Option<Message> {
    if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Message::Quit);
    }
    if key.code == KeyCode::Char('l') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Some(Message::ToggleLanguage);
    }

    let key = match app.dialogs.active_mut() {
        Some(dialog) => match dialog.handle_key(key) {
            DialogResult::Consumed => {
                return None;
            }

            DialogResult::Message(message) => {
                return Some(message);
            }

            DialogResult::Unhandled(key) => key,
        },

        None => key,
    };

    if let Some(message) = map_key_base(app, key) {
        return Some(message);
    }

    map_key_page(app, key)
}

fn map_key_base(app: &App, key: KeyEvent) -> Option<Message> {
    if !app.dialogs.is_empty() {
        return None;
    }

    match key.code {
        KeyCode::Esc => Some(Message::OpenDialog {
            dialog: Box::new(preset::application::new()),
            mode: crate::app::DialogOpenMode::Push,
        }),
        KeyCode::Char('1') => Some(Message::Navigate(Page::Home)),
        KeyCode::Char('2') => Some(Message::Navigate(Page::Users)),
        KeyCode::Char('3') => Some(Message::Navigate(Page::Transactions)),
        KeyCode::Char('4') => Some(Message::Navigate(Page::Stats)),
        _ => None,
    }
}

fn map_key_page(app: &App, key: KeyEvent) -> Option<Message> {
    match app.page {
        Page::Home => map_key_home(key),
        Page::Users => map_key_users(key),
        Page::Transactions => map_key_transactions(key),
        Page::Stats => map_key_stats(key),
    }
}
fn map_key_home(_key: KeyEvent) -> Option<Message> {
    None
}

fn map_key_users(_key: KeyEvent) -> Option<Message> {
    None
}

fn map_key_transactions(_key: KeyEvent) -> Option<Message> {
    None
}

fn map_key_stats(_key: KeyEvent) -> Option<Message> {
    None
}
