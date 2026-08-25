use crate::app::dialog::DialogResult;
use crate::app::dialog::menu::preset;
use crate::app::{App, Message, PageId};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(crate) fn map_key(app: &mut App, key: KeyEvent) -> Option<Message> {
    if let Some(message) = map_key_global(key) {
        return Some(message);
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

    app.page.handle_key(key)
}

fn map_key_global(key: KeyEvent) -> Option<Message> {
    if !key.modifiers.contains(KeyModifiers::CONTROL) {
        return None;
    }

    match key.code {
        KeyCode::Char('l') => Some(Message::ToggleLanguage),
        _ => None,
    }
}

fn map_key_base(app: &App, key: KeyEvent) -> Option<Message> {
    if !app.dialogs.is_empty() {
        return None;
    }

    if let Some(page) = PageId::from_key(key.code) {
        return Some(Message::Navigate(page));
    }

    match key.code {
        KeyCode::Esc => Some(Message::OpenDialog {
            dialog: Box::new(preset::application::new()),
            mode: crate::app::DialogOpenMode::Push,
        }),
        _ => None,
    }
}
