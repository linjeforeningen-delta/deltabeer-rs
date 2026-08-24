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

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Home => "Home",
            Self::Users => "Users",
            Self::Transactions => "Transactions",
            Self::Stats => "Stats",
        }
    }
}