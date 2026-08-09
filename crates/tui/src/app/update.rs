use crate::app::{App, Message};

impl App {
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Quit => {
                self.should_quit = true;
            }
        }
    }
}