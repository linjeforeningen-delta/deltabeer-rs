use crate::app::TextInput;

pub(crate) struct AdminAuthDialogState {
    pub card: Option<String>,
    pub password: TextInput,
}

impl AdminAuthDialogState {
    pub fn handle_scan(&mut self, card: String) -> Result<(), String> {
        self.card = Some(card);
        Ok(())
    }
}
