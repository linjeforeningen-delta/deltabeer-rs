use crate::api::models::user::UserId;
use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{GrantAdminDialog, MakeUserDialog, MenuDialog, RevokeAdminDialog};

pub(crate) fn admin_menu(user_id: &UserId) -> MenuDialog {
    MenuDialog::new(
        "Admin",
        vec![
            MenuOption::new("Make User", 'M', || Box::new(MakeUserDialog::new())),
            MenuOption::new("Grant Admin", 'G', || Box::new(GrantAdminDialog::new())),
            MenuOption::new("Revoke Admin", 'R', || Box::new(RevokeAdminDialog::new())),
        ],
    )
}
