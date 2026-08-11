use crate::api::models::user::User;
use crate::app::NumericInput;


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

pub(crate) enum DialogOpenMode {
    Push,
    ReplaceTop,
    Reset,
}