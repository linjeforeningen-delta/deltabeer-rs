use crate::app::dialog::DialogResult;
use crate::app::dialog::menu::preset;
use crate::app::{App, Message, PageId};
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

    app.page.handle_key(key)
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
        KeyCode::Char('1') => Some(Message::Navigate(PageId::Home)),
        KeyCode::Char('2') => Some(Message::Navigate(PageId::Users)),
        KeyCode::Char('3') => Some(Message::Navigate(PageId::Transactions)),
        KeyCode::Char('4') => Some(Message::Navigate(PageId::Stats)),
        _ => None,
    }
}
