use crate::api::models::user::UserId;
use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{MakeUserDialog, MenuDialog};

pub(crate) fn admin_menu(user_id: &UserId) -> MenuDialog {
    MenuDialog::new(
        "Admin",
        vec![MenuOption::new("Make User", 'M', || {
            Box::new(MakeUserDialog::new())
        })],
    )
}
