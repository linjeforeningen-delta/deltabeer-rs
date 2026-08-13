use super::Dialog;

pub(crate) enum DialogOpenMode {
    Push,
    ReplaceTop,
    Reset,
}

pub(crate) struct DialogStack {
    stack: Vec<Dialog>,
}

impl DialogStack {
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub(crate) fn open(&mut self, dialog: Dialog, mode: DialogOpenMode) {
        match mode {
            DialogOpenMode::Push => {
                self.stack.push(dialog);
            }
            DialogOpenMode::ReplaceTop => {
                self.stack.pop();
                self.stack.push(dialog);
            }
            DialogOpenMode::Reset => {
                self.stack.clear();
                self.stack.push(dialog);
            }
        }
    }

    pub(crate) fn close(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn active(&self) -> Option<&Dialog> {
        self.stack.last()
    }

    pub(crate) fn active_mut(&mut self) -> Option<&mut Dialog> {
        self.stack.last_mut()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
