pub(crate) mod dialog;
mod option;
pub(crate) mod preset;
pub(crate) use dialog::{MenuKind, MenuTitle};
pub(crate) use option::{Language, MenuLabel, MenuOption, set_locale, toggle_locale};
