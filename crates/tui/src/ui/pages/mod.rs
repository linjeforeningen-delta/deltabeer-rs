use crate::app::{App, Page};
use crate::ui::theme::Theme;
use ratatui::{Frame, layout::Rect};

pub(crate) mod home;
pub(crate) mod stats;
pub(crate) mod transactions;
pub(crate) mod users;

impl Page {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
        match self {
            Page::Home(page) => page.draw(frame, area, app, theme),
            Page::Users(page) => page.draw(frame, area, app, theme),
            Page::Transactions(page) => page.draw(frame, area, app, theme),
            Page::Stats(page) => page.draw(frame, area, app, theme),
        }
    }
}
