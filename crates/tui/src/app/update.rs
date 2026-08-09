use crate::app::{App, Message, Page};

impl App {
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::Quit => self.should_quit = true,

            Message::OpenHome => self.page = Page::Home,
            Message::OpenUsers => self.page = Page::Users,
            Message::OpenTransactions => {
                self.page = Page::Transactions;
            }
            Message::OpenStats => self.page = Page::Stats,
        }
    }
}