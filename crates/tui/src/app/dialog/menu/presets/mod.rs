use crate::api::models::user::UserId;
use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{
    GrantAdminDialog, MakeUserDialog, MenuDialog, RevokeAdminDialog, TopUpDialog, UpdateUserDialog,
};

pub(crate) fn admin_menu(user_id: &UserId) -> MenuDialog {
    MenuDialog::new_admin(
        "Admin",
        vec![
            MenuOption::new("Top Up", 'T', || Box::new(TopUpDialog::new())),
            MenuOption::new("Make User", 'M', || Box::new(MakeUserDialog::new())),
            MenuOption::new("Update User", 'U', || Box::new(UpdateUserDialog::new())),
            MenuOption::new("Grant Admin", 'G', || Box::new(GrantAdminDialog::new())),
            MenuOption::new("Revoke Admin", 'R', || Box::new(RevokeAdminDialog::new())),
        ],
    )
}
