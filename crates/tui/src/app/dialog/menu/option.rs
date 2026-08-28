use crate::app::Message;
use std::fmt;

pub(crate) struct MenuOption {
    pub(crate) label: MenuLabel,
    pub(crate) key: char,
    pub(crate) message: Box<dyn Fn() -> Message>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum MenuLabel {
    About,
    ChangeLanguage,
    Quit,
    English,
    NorwegianBokmaal,
    TopUp,
    MakeUser,
    UpdateUser,
    GrantAdmin,
    RevokeAdmin,
    Login,
    Logout,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Language {
    English,
    NorwegianBokmaal,
}

impl Language {
    pub(crate) const fn locale(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::NorwegianBokmaal => "nb",
        }
    }
}

pub(crate) fn set_locale(language: Language) {
    rust_i18n::set_locale(language.locale());
}

pub(crate) fn toggle_locale() {
    let language = if &*rust_i18n::locale() == "nb" {
        Language::English
    } else {
        Language::NorwegianBokmaal
    };
    set_locale(language);
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
