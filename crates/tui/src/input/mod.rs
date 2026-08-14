mod keyboard;
mod scanner;

use crate::{
    app::{App, Message},
    input::scanner::{Scanner, ScannerResult},
};
use crossterm::event::KeyEvent;

pub(crate) struct Input {
    scanner: Scanner,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self {
            scanner: Scanner::new(),
        }
    }

    pub(crate) fn handle(
        &mut self,
        app: &mut App,
        key: KeyEvent,
    ) -> Vec<Message> {
        match self.scanner.handle(key) {
            ScannerResult::Waiting => {
                vec![]
            }

            ScannerResult::Scanned(card) => {
                vec![Message::CardScanned(card)]
            }

            ScannerResult::NotScan(keys) => {
                keys.into_iter()
                    .filter_map(|key| keyboard::map_key(app, key))
                    .collect()
            }
        }
    }

    pub(crate) fn tick(
        &mut self,
        app: &mut App,
    ) -> Vec<Message> {
        match self.scanner.flush() {
            ScannerResult::NotScan(keys) => keys
                .into_iter()
                .filter_map(|key| keyboard::map_key(app, key))
                .collect(),

            _ => vec![],
        }
    }
}