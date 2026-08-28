use super::{AdminDialog, Dialog};
use crate::auth::{AdminContext, AuthState};

#[derive(Debug)]
pub(crate) enum DialogOpenMode {
    Push,
    ReplaceTop,
    Reset,
}

pub(crate) struct DialogStack {
    stack: Vec<DialogEntry>,
}

enum DialogEntry {
    Normal(Box<dyn Dialog>),
    Admin(Box<dyn AdminDialog>),
}

impl DialogStack {
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub(crate) fn open(&mut self, dialog: Box<dyn Dialog>, mode: DialogOpenMode) {
        self.open_entry(DialogEntry::Normal(dialog), mode);
    }

    pub(crate) fn open_admin(
        &mut self,
        dialog: Box<dyn AdminDialog>,
        mode: DialogOpenMode,
        state: &AuthState,
        context: Option<AdminContext>,
    ) {
        let mut dialog = dialog;
        dialog.set_auth_state(state);
        dialog.set_admin_context(context);

        self.open_entry(DialogEntry::Admin(dialog), mode);
    }

    fn open_entry(&mut self, entry: DialogEntry, mode: DialogOpenMode) {
        match mode {
            DialogOpenMode::Push => {
                self.stack.push(entry);
            }
            DialogOpenMode::ReplaceTop => {
                self.stack.pop();
                self.stack.push(entry);
            }
            DialogOpenMode::Reset => {
                self.stack.clear();
                self.stack.push(entry);
            }
        }
    }

    pub(crate) fn set_auth_state(&mut self, state: &AuthState) {
        if let Some(DialogEntry::Admin(dialog)) = self.stack.last_mut() {
            dialog.set_auth_state(state);
        }
    }

    pub(crate) fn close(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn close_to_admin_menu(&mut self) {
        while !matches!(
            self.stack.last(),
            Some(DialogEntry::Admin(dialog)) if dialog.is_admin_menu()
        ) {
            if self.stack.pop().is_none() {
                break;
            }
        }
    }

    pub(crate) fn is_admin_menu_active(&self) -> bool {
        matches!(
            self.stack.last(),
            Some(DialogEntry::Admin(dialog)) if dialog.is_admin_menu()
        )
    }

    pub(crate) fn clear(&mut self) {
        self.stack.clear();
    }

    pub(crate) fn active(&self) -> Option<&(dyn Dialog + '_)> {
        match self.stack.last()? {
            DialogEntry::Normal(dialog) => Some(dialog.as_ref()),
            DialogEntry::Admin(dialog) => Some(dialog.as_ref()),
        }
    }

    pub(crate) fn active_mut(&mut self) -> Option<&mut (dyn Dialog + '_)> {
        match self.stack.last_mut()? {
            DialogEntry::Normal(dialog) => Some(dialog.as_mut()),
            DialogEntry::Admin(dialog) => Some(dialog.as_mut()),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
