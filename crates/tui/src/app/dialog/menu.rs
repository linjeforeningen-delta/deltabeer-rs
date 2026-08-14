use crossterm::event::{KeyCode, KeyEvent};
use std::fmt;
use std::fmt::Debug;

use crate::app::dialog::{DialogBehavior, DialogResult};
use crate::app::{
    Dialog,
    DialogOpenMode,
    Message,
};

pub(crate) struct MenuOption {
    pub(crate) name: String,
    pub(crate) key: char,
    pub(crate) next: Box<dyn Fn() -> Box<dyn Dialog>>,
}

impl fmt::Debug for MenuOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MenuOption")
            .field("name", &self.name)
            .field("key", &self.key)
            .finish()
    }
}


impl MenuOption {
    pub(crate) fn new<F>(
        name: impl Into<String>,
        key: char,
        next: F,
    ) -> Self
    where
        F: Fn() -> Box<dyn Dialog> + 'static,
    {
        Self {
            name: name.into(),
            key,
            next: Box::new(next),
        }
    }
}


#[derive(Debug)]
pub(crate) struct MenuDialog {
    pub(crate) title: String,
    pub(crate) options: Vec<MenuOption>,
}

impl MenuDialog {
    pub(crate) fn new(
        title: impl Into<String>,
        options: Vec<MenuOption>,
    ) -> Self {
        Self {
            title: title.into(),
            options,
        }
    }
}


impl DialogBehavior for MenuDialog {
    fn handle_key_inner(
        &mut self,
        key: KeyEvent,
    ) -> DialogResult<KeyEvent> {
        let KeyCode::Char(char) = key.code else {
            return DialogResult::Unhandled(key);
        };

        let Some(option) = self
            .options
            .iter()
            .find(|option| {
                option.key.eq_ignore_ascii_case(&char)
            })
        else {
            return DialogResult::Unhandled(key);
        };

        DialogResult::Message(
            Message::OpenDialog {
                dialog: (option.next)(),
                mode: DialogOpenMode::ReplaceTop,
            },
        )
    }
}