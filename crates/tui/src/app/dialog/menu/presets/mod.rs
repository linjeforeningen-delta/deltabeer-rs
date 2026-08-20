use crate::api::models::user::UserId;
use crate::app::dialog::menu::option::MenuOption;
use crate::app::dialog::{
    GrantAdminDialog, MakeUserDialog, MenuDialog, RevokeAdminDialog, TopUpDialog, UpdateUserDialog,
};
use crate::app::{DialogOpenMode, Message};

pub(crate) fn admin_menu(user_id: &UserId) -> MenuDialog {
    MenuDialog::new_admin(
        "Admin",
        vec![
            MenuOption::new("Top Up", 'T', || Message::OpenDialog {
                dialog: Box::new(TopUpDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Make User", 'M', || Message::OpenDialog {
                dialog: Box::new(MakeUserDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Update User", 'U', || Message::OpenDialog {
                dialog: Box::new(UpdateUserDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Grant Admin", 'G', || Message::OpenDialog {
                dialog: Box::new(GrantAdminDialog::new()),
                mode: DialogOpenMode::Push,
            }),
            MenuOption::new("Revoke Admin", 'R', || Message::OpenDialog {
                dialog: Box::new(RevokeAdminDialog::new()),
                mode: DialogOpenMode::Push,
            }),
        ],
    )
}
