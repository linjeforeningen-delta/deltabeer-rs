use crate::app::page::PageResult;
use crossterm::event::KeyEvent;

#[derive(Debug)]
pub(crate) struct StatsPage;

impl StatsPage {
    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> PageResult<KeyEvent> {
        PageResult::Unhandled(key)
    }
}
