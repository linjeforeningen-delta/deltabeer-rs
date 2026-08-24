use crate::app::Message;
use std::fmt;

pub(crate) struct MenuOption {
    pub(crate) label: MenuLabel,
    pub(crate) key: char,
    pub(crate) message: Box<dyn Fn() -> Message>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MenuLabel {
    TopUp,
    MakeUser,
    UpdateUser,
    GrantAdmin,
    RevokeAdmin,
    Login,
    Logout,
}

impl fmt::Debug for MenuOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MenuOption")
            .field("label", &self.label)
            .field("key", &self.key)
            .finish()
    }
}

impl MenuOption {
    pub(crate) fn new<F>(label: MenuLabel, key: char, message: F) -> Self
    where
        F: Fn() -> Message + 'static,
    {
        Self {
            label,
            key,
            message: Box::new(message),
        }
    }
}
