use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Debug;

use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::{DialogOpenMode, Message};

#[derive(Debug)]
pub(crate) struct MenuDialog {
    pub(crate) title: String,
    pub(crate) options: Vec<MenuOption>,
    pub(crate) is_admin: bool,
}

impl MenuDialog {
    pub(crate) fn new(title: impl Into<String>, options: Vec<MenuOption>) -> Self {
        Self {
            title: title.into(),
            options,
            is_admin: false,
        }
    }

    pub(crate) fn new_admin(title: impl Into<String>, options: Vec<MenuOption>) -> Self {
        Self {
            title: title.into(),
            options,
            is_admin: true,
        }
    }

    pub(crate) fn admin(mut self) -> Self {
        self.is_admin = true;
        self
    }
}

impl DialogBehavior for MenuDialog {
    fn handle_key_inner(&mut self, key: KeyEvent) -> DialogResult<KeyEvent> {
        let KeyCode::Char(char) = key.code else {
            return DialogResult::Unhandled(key);
        };

        let Some(option) = self
            .options
            .iter()
            .find(|option| option.key.eq_ignore_ascii_case(&char))
        else {
            return DialogResult::Unhandled(key);
        };

        DialogResult::Message(Message::OpenDialog {
            dialog: (option.next)(),
            mode: DialogOpenMode::Push,
        })
    }
}
