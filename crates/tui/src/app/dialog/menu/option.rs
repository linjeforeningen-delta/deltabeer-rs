use crate::app::Dialog;
use std::fmt;

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
    pub(crate) fn new<F>(name: impl Into<String>, key: char, next: F) -> Self
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
