use crate::api::models::user::User;
use crate::app::NumericInput;
use crate::auth::AuthState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Home,
    Users,
    Transactions,
    Stats,
}

pub(crate) enum Dialog {
    User(UserDialogState),
}

impl Dialog {
    pub(crate) fn numeric_input_mut(&mut self) -> Option<&mut NumericInput> {
        match self {
            Dialog::User(state) => Some(&mut state.amount),
            _ => None,
        }
    }
}

pub(crate) struct UserDialogState {
    pub user: User,
    pub amount: NumericInput,
}

pub(crate) struct App {
    pub auth: AuthState,
    pub page: Page,
    pub dialog: Option<Dialog>,
    pub status: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            page: Page::Home,
            dialog: None,
            status: "Ready for card".into(),
            should_quit: false,
        }
    }
}