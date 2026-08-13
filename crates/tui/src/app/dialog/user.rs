use crate::api::models::user::User;
use crate::app::TextInput;

pub(crate) struct UserDialogState {
    pub user: User,
    pub amount: TextInput,
}
