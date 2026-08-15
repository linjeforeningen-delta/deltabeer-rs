use crossterm::event::KeyEvent;
use std::time::{Duration, Instant};

pub(crate) enum ScannerResult {
    Waiting,
    Scanned(String),
    NotScan(Vec<KeyEvent>),
}

pub(crate) struct Scanner {
    buffer: Vec<KeyEvent>,
    last_input: Option<Instant>,
}

impl Scanner {
    const MAX_GAP: Duration = Duration::from_millis(80);

    pub(crate) fn new() -> Self {
        Self {
            buffer: Vec::new(),
            last_input: None,
        }
    }

    pub(crate) fn handle(&mut self, key: KeyEvent) -> ScannerResult {
        let now = Instant::now();

        match key.code {
            crossterm::event::KeyCode::Char(c) if c.is_ascii_digit() => {
                self.buffer.push(key);
                self.last_input = Some(now);

                ScannerResult::Waiting
            }

            crossterm::event::KeyCode::Enter if !self.buffer.is_empty() => {
                let keys = std::mem::take(&mut self.buffer);

                self.last_input = None;

                let card: String = keys
                    .iter()
                    .filter_map(|key| match key.code {
                        crossterm::event::KeyCode::Char(c) => Some(c),
                        _ => None,
                    })
                    .collect();

                ScannerResult::Scanned(card)
            }

            _ if !self.buffer.is_empty() => {
                let mut keys = std::mem::take(&mut self.buffer);
                keys.push(key);

                self.last_input = None;

                ScannerResult::NotScan(keys)
            }

            _ => ScannerResult::NotScan(vec![key]),
        }
    }

    pub(crate) fn flush(&mut self) -> ScannerResult {
        if self.buffer.is_empty() {
            return ScannerResult::Waiting;
        }

        let Some(last) = self.last_input else {
            return ScannerResult::Waiting;
        };

        if last.elapsed() <= Self::MAX_GAP {
            return ScannerResult::Waiting;
        }

        let keys = std::mem::take(&mut self.buffer);

        self.last_input = None;

        ScannerResult::NotScan(keys)
    }
}
