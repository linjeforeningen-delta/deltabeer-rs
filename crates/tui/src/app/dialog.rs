use crate::api::models::user::User;
use crate::app::TextInput;


pub(crate) enum Dialog {
    User(UserDialogState),
}

impl Dialog {
    pub(crate) fn input_mut(&mut self) -> Option<&mut TextInput> {
        match self {
            Dialog::User(state) => Some(&mut state.amount),
            _ => None,
        }
    }
}

pub(crate) struct UserDialogState {
    pub user: User,
    pub amount: TextInput,
}

pub(crate) enum DialogOpenMode {
    Push,
    ReplaceTop,
    Reset,
}