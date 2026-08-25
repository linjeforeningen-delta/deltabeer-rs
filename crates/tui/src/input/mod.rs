mod keyboard;
mod scanner;

use crate::{
    app::{App, Message},
    input::scanner::{Scanner, ScannerResult},
};
use crossterm::event::KeyEvent;
use std::time::Duration;

pub(crate) struct Input {
    scanner: Scanner,
}

impl Input {
    pub(crate) fn new(scanner_max_gap: Duration) -> Self {
        Self {
            scanner: Scanner::new(scanner_max_gap),
        }
    }

    pub(crate) fn handle(&mut self, app: &mut App, key: KeyEvent) -> Vec<Message> {
        match self.scanner.handle(key) {
            ScannerResult::Waiting => {
                vec![]
            }

            ScannerResult::Scanned(card) => {
                vec![Message::CardScanned(card)]
            }

            ScannerResult::NotScan(keys) => Self::map_keys(app, keys),
        }
    }

    pub(crate) fn tick(&mut self, app: &mut App) -> Vec<Message> {
        match self.scanner.flush() {
            ScannerResult::NotScan(keys) => Self::map_keys(app, keys),

            _ => vec![],
        }
    }

    fn map_keys(app: &mut App, keys: Vec<KeyEvent>) -> Vec<Message> {
        keys.into_iter()
            .filter_map(|key| keyboard::map_key(app, key))
            .collect()
    }
}
