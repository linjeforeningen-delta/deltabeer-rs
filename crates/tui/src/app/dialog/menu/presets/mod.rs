use crate::api::models::user::UserId;
use crate::app::dialog::MenuDialog;

pub(crate) fn admin_menu(user_id: &UserId) -> MenuDialog {
    MenuDialog::new(
        "Admin",
        vec![],
    )
}