use crate::app::command::Command;
use crate::app::state::{Dialog, UserDialogState};
use crate::app::{App, Message, Page};

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Option<Command> {
        match message {
            Message::Quit => {
                self.should_quit = true;
                None
            }

            Message::OpenHome => {
                self.page = Page::Home;
                None
            }

            Message::OpenUsers => {
                self.page = Page::Users;
                None
            }

            Message::OpenTransactions => {
                self.page = Page::Transactions;
                None
            }

            Message::OpenStats => {
                self.page = Page::Stats;
                None
            }

            Message::CardScanned(card) => {
                Some(Command::LookupUser(card))
            }

            Message::UserLoaded(user) => {
                self.dialog = Some(Dialog::User(UserDialogState {
                    user,
                    amount: String::new(),
                }));

                self.status = "User loaded".into();
                None
            }

            Message::UserLoadFailed(error) => {
                self.status = error;
                None
            }

            Message::CloseDialog => {
                self.dialog = None;
                None
            }
        }
    }
}