use crate::app::{App, Page};
use crate::ui::theme::{Palette, Theme};
use ratatui::{Frame, layout::Rect};

pub(crate) mod home;
pub(crate) mod stats;
pub(crate) mod transactions;
pub(crate) mod users;

pub(crate) fn page_palette(app: &App, theme: &Theme) -> Palette {
    if app.dialogs.active().is_some() {
        theme.dimmed()
    } else {
        theme.active(&app.auth)
    }
}

impl Page {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        match self {
            Page::Home(page) => page.draw(frame, area, palette),
            Page::Users(page) => page.draw(frame, area, palette),
            Page::Transactions(page) => page.draw(frame, area, palette),
            Page::Stats(page) => page.draw(frame, area, palette),
        }
    }
}
