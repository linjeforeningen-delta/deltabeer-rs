use crate::api::request::ApiRequest;
use crate::app::dialog::MenuDialog;
use crate::app::dialog::menu::{MenuKind, MenuLabel, MenuOption, MenuTitle};
use crate::app::{DialogOpenMode, Message};

pub(crate) fn new() -> MenuDialog {
    MenuDialog::new(
        MenuTitle::Admin,
        options(false),
        MenuKind::Admin { logged_in: false },
    )
}

pub(crate) fn options(logged_in: bool) -> Vec<MenuOption> {
    vec![
        MenuOption::new(MenuLabel::TopUp, 'T', || Message::OpenDialog {
            dialog: Box::new(crate::app::dialog::TopUpDialog::new()),
            mode: DialogOpenMode::Push,
        }),
        MenuOption::new(MenuLabel::MakeUser, 'M', || Message::OpenDialog {
            dialog: Box::new(crate::app::dialog::MakeUserDialog::new()),
            mode: DialogOpenMode::Push,
        }),
        MenuOption::new(MenuLabel::UpdateUser, 'U', || Message::OpenDialog {
            dialog: Box::new(crate::app::dialog::UpdateUserDialog::new()),
            mode: DialogOpenMode::Push,
        }),
        MenuOption::new(MenuLabel::GrantAdmin, 'G', || Message::OpenDialog {
            dialog: Box::new(crate::app::dialog::GrantAdminDialog::new()),
            mode: DialogOpenMode::Push,
        }),
        MenuOption::new(MenuLabel::RevokeAdmin, 'R', || Message::OpenDialog {
            dialog: Box::new(crate::app::dialog::RevokeAdminDialog::new()),
            mode: DialogOpenMode::Push,
        }),
        if logged_in {
            MenuOption::new(MenuLabel::Logout, 'L', || {
                Message::ApiRequest(ApiRequest::EndAdminSession)
            })
        } else {
            MenuOption::new(MenuLabel::Login, 'L', || Message::OpenAdminDialog {
                dialog: Box::new(crate::app::dialog::AdminSessionLoginDialog::new()),
                mode: DialogOpenMode::Push,
            })
        },
    ]
}
