use crossterm::event::{KeyCode, KeyEvent};
use std::fmt::Debug;

use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{DialogBehavior, DialogResult};

#[derive(Debug)]
pub(crate) struct MenuDialog {
    pub(crate) title: MenuTitle,
    pub(crate) options: Vec<MenuOption>,
    pub(crate) is_admin: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MenuTitle {
    Admin,
}

impl MenuDialog {
    pub(crate) fn new_admin(title: MenuTitle, options: Vec<MenuOption>) -> Self {
        Self {
            title: title.into(),
            options,
            is_admin: true,
        }
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

        DialogResult::Message((option.message)())
    }
}
