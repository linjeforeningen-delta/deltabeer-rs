use crate::app::Message;
use std::fmt;

pub(crate) struct MenuOption {
    pub(crate) name: String,
    pub(crate) key: char,
    pub(crate) message: Box<dyn Fn() -> Message>,
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
    pub(crate) fn new<F>(name: impl Into<String>, key: char, message: F) -> Self
    where
        F: Fn() -> Message + 'static,
    {
        Self {
            name: name.into(),
            key,
            message: Box::new(message),
        }
    }
}
