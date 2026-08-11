use crate::app::Dialog;
use crate::app::dialog::DialogOpenMode;
use crate::auth::AuthState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Home,
    Users,
    Transactions,
    Stats,
}


pub(crate) struct App {
    pub auth: AuthState,
    pub page: Page,
    pub dialogs: Vec<Dialog>,
    pub status: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            page: Page::Home,
            dialogs: Vec::new(),
            status: "Ready for card".into(),
            should_quit: false,
        }
    }

    pub(crate) fn open(
        &mut self,
        dialog: Dialog,
        mode: DialogOpenMode,
    ) {
        match mode {
            DialogOpenMode::Push => {
                self.dialogs.push(dialog);
            }

            DialogOpenMode::ReplaceTop => {
                self.dialogs.pop();
                self.dialogs.push(dialog);
            }

            DialogOpenMode::Reset => {
                self.dialogs.clear();
                self.dialogs.push(dialog);
            }
        }
    }
}