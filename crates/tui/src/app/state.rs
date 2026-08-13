use crate::app::dialog::DialogStack;
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
    pub dialogs: DialogStack,
    pub status: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            page: Page::Home,
            dialogs: DialogStack::new(),
            status: "Ready for card".into(),
            should_quit: false,
        }
    }
}