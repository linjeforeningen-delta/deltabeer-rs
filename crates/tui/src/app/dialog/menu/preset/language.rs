use crate::app::Message;
use crate::app::dialog::MenuDialog;
use crate::app::dialog::menu::{Language, MenuKind, MenuLabel, MenuOption, MenuTitle};

pub(crate) fn new() -> MenuDialog {
    MenuDialog::new(
        MenuTitle::Language,
        vec![
            MenuOption::new(MenuLabel::English, 'E', || {
                Message::SetLanguage(Language::English)
            }),
            MenuOption::new(MenuLabel::NorwegianBokmaal, 'B', || {
                Message::SetLanguage(Language::NorwegianBokmaal)
            }),
            MenuOption::new(MenuLabel::NorwegianNynorsk, 'N', || {
                Message::SetLanguage(Language::NorwegianNynorsk)
            }),
        ],
        MenuKind::Normal,
    )
}
