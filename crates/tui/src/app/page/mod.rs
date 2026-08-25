mod home;
mod stats;
mod transactions;
mod users;

use crate::app::Message;
use crossterm::event::{KeyCode, KeyEvent};
pub(crate) use home::HomePage;
pub(crate) use stats::StatsPage;
pub(crate) use transactions::TransactionsPage;
pub(crate) use users::UsersPage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PageId {
    Home,
    Users,
    Transactions,
    Stats,
}

impl PageId {
    pub(crate) const ALL: [Self; 4] = [Self::Home, Self::Users, Self::Transactions, Self::Stats];

    pub(crate) fn label(self) -> String {
        match self {
            Self::Home => t!("nav.home").to_string(),
            Self::Users => t!("nav.users").to_string(),
            Self::Transactions => t!("nav.transactions").to_string(),
            Self::Stats => t!("nav.stats").to_string(),
        }
    }
    pub(crate) fn from_key(code: KeyCode) -> Option<Self> {
        match code {
            KeyCode::Char('1') => Some(Self::Home),
            KeyCode::Char('2') => Some(Self::Users),
            KeyCode::Char('3') => Some(Self::Transactions),
            KeyCode::Char('4') => Some(Self::Stats),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum Page {
    Home(HomePage),
    Users(UsersPage),
    Transactions(TransactionsPage),
    Stats(StatsPage),
}

impl Page {
    pub(crate) fn id(&self) -> PageId {
        match self {
            Self::Home(_) => PageId::Home,
            Self::Users(_) => PageId::Users,
            Self::Transactions(_) => PageId::Transactions,
            Self::Stats(_) => PageId::Stats,
        }
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> Option<Message> {
        match self {
            Self::Home(page) => page.handle_key(key),
            Self::Users(page) => page.handle_key(key),
            Self::Transactions(page) => page.handle_key(key),
            Self::Stats(page) => page.handle_key(key),
        }
    }
}

impl From<PageId> for Page {
    fn from(id: PageId) -> Self {
        match id {
            PageId::Home => Self::Home(HomePage),
            PageId::Users => Self::Users(UsersPage),
            PageId::Transactions => Self::Transactions(TransactionsPage),
            PageId::Stats => Self::Stats(StatsPage),
        }
    }
}
