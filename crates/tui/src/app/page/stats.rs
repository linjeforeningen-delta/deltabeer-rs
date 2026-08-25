use crate::app::Message;
use crossterm::event::KeyEvent;

#[derive(Debug)]
pub(crate) struct StatsPage;

impl StatsPage {
    pub(crate) fn handle_key(&mut self, _key: KeyEvent) -> Option<Message> {
        None
    }
}