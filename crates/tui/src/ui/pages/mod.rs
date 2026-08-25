use crate::app::App;
use crate::ui::theme::Theme;
use ratatui::Frame;
use ratatui::layout::Rect;

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


    pub(crate) fn draw(
        self,
        frame: &mut Frame,
        area: Rect,
        app: &App,
        theme: &Theme,
    ) {
        match self {
            Self::Home => home::draw(frame, area, app, theme),
            Self::Users => users::draw(frame, area, app, theme),
            Self::Transactions => transactions::draw(frame, area, app, theme),
            Self::Stats => stats::draw(frame, area, app, theme),
        }
    }
}
