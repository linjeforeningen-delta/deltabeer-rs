use crate::auth::AuthState;

pub(crate) struct App {
    pub auth: AuthState,
    pub status: String,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            auth: AuthState::Normal,
            status: "Ready for card".into(),
            should_quit: false,
        }
    }
}