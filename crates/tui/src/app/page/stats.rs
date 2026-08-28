use crate::api::request::ApiRequest;
use crate::app::{Message, page::PageResult};
use crate::model::Stats;
use crossterm::event::KeyEvent;

#[derive(Debug)]
pub(crate) struct StatsPage {
    pub(crate) stats: Option<Stats>,
    pub(crate) loading: bool,
}

impl StatsPage {
    pub(crate) fn new() -> Self {
        Self {
            stats: None,
            loading: true,
        }
    }

    pub(crate) fn set_stats(&mut self, stats: Stats) {
        self.stats = Some(stats);
        self.loading = false;
    }

    pub(crate) fn finish_loading(&mut self) {
        self.loading = false;
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> PageResult<KeyEvent> {
        if key.code == crossterm::event::KeyCode::Char('r') {
            self.loading = true;
            return PageResult::Message(Message::ApiRequest(ApiRequest::Stats));
        }
        PageResult::Unhandled(key)
    }
}
