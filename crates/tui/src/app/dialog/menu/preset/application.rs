use crate::app::dialog::MenuDialog;
use crate::app::dialog::menu::{MenuKind, MenuLabel, MenuOption, MenuTitle};
use crate::app::{DialogOpenMode, Message};

pub(crate) fn new() -> MenuDialog {
    MenuDialog::new(
        MenuTitle::Application,
        vec![
            MenuOption::new(MenuLabel::ChangeLanguage, 'L', || Message::OpenDialog {
                dialog: Box::new(super::language::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new(MenuLabel::Quit, 'Q', || Message::Quit),
        ],
        MenuKind::Normal,
    )
}
