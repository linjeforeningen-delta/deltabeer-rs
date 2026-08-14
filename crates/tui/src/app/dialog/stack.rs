use super::Dialog;

#[derive(Debug)]
pub(crate) enum DialogOpenMode {
    Push,
    ReplaceTop,
    Reset,
}

pub(crate) struct DialogStack {
    stack: Vec<Box<dyn Dialog>>,
}

impl DialogStack {
    pub(crate) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub(crate) fn open(&mut self, dialog: Box<dyn Dialog>, mode: DialogOpenMode) {
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

    pub(crate) fn active(&self) -> Option<&(dyn Dialog + '_)> {
        let dialog = self.stack.last()?;
        Some(dialog.as_ref())
    }

    pub(crate) fn active_mut(&mut self) -> Option<&mut (dyn Dialog + '_)> {
        let dialog = self.stack.last_mut()?;
        Some(dialog.as_mut())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
