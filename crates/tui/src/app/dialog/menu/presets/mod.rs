use crate::app::dialog::menu::{Language, MenuLabel, MenuOption, MenuTitle};
use crate::app::dialog::{AdminMenuDialog, MenuDialog};
use crate::app::{DialogOpenMode, Message};

pub(crate) fn admin_menu() -> AdminMenuDialog {
    AdminMenuDialog::new()
}

pub(crate) fn application_menu() -> MenuDialog {
    MenuDialog::new(
        MenuTitle::Application,
        vec![
            MenuOption::new(MenuLabel::ChangeLanguage, 'L', || Message::OpenDialog {
                dialog: Box::new(language_menu()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new(MenuLabel::Quit, 'Q', || Message::Quit),
        ],
    )
}

fn language_menu() -> MenuDialog {
    MenuDialog::new(
        MenuTitle::Language,
        vec![
            MenuOption::new(MenuLabel::English, 'E', || {
                Message::SetLanguage(Language::English)
            }),
            MenuOption::new(MenuLabel::NorwegianBokmaal, 'N', || {
                Message::SetLanguage(Language::NorwegianBokmaal)
            }),
        ],
    )
}
