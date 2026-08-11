use crate::app::command::Command;
use crate::app::state::{Dialog, UserDialogState};
use crate::app::{App, Message, NumericInput, Page};

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
                    amount: NumericInput::new(),
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

            Message::NumericInput(c) => {
                if let Some(dialog) = &mut self.dialog {
                    if let Some(input) = dialog.numeric_input_mut() {
                        input.push(c);
                    }
                }

                None
            }

            Message::NumericBackspace => {
                if let Some(dialog) = &mut self.dialog {
                    if let Some(input) = dialog.numeric_input_mut() {
                        input.backspace();
                    }
                }

                None
            }
            Message::Submit => {
                match &mut self.dialog {
                    Some(Dialog::User(state)) => {
                        todo!("Implement spend command");
                    }

                    _ => None,
                }
            }
        }
    }
}