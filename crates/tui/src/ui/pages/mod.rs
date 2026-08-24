pub(crate) mod home;
pub(crate) mod stats;
pub(crate) mod transactions;
pub(crate) mod users;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Page {
    Home,
    Users,
    Transactions,
    Stats,
}

impl Page {
    pub(crate) const ALL: [Self; 4] = [Self::Home, Self::Users, Self::Transactions, Self::Stats];

    pub(crate) fn label(self) -> String {
        match self {
            Self::Home => t!("nav.home").to_string(),
            Self::Users => t!("nav.users").to_string(),
            Self::Transactions => t!("nav.transactions").to_string(),
            Self::Stats => t!("nav.stats").to_string(),
        }
    }
}
