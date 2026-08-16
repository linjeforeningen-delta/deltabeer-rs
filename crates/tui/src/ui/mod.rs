pub(crate) mod dialogs;
mod helpers;
pub(crate) mod layout;
pub mod pages;
pub(crate) mod theme;

use crate::app::{App, Page};
use crate::ui::theme::THEME;
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(frame: &mut Frame, app: &App) {
    let theme = THEME;
    let palette = theme.active(&app.auth);
    let areas = layout::app_layout(frame.area());

    let header = Block::default()
        .title(" DeltaBeer ")
        .title_style(theme.title_style(palette))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(palette.border());

    frame.render_widget(header, areas.header);

    match app.page {
        Page::Home => {
            pages::home::draw(frame, areas.body, app, &theme);
        }
        Page::Users => {
            pages::users::draw(frame, areas.body, app, &theme);
        }
        Page::Transactions => {
            pages::transactions::draw(frame, areas.body, app, &theme);
        }
        Page::Stats => {
            pages::stats::draw(frame, areas.body, app, &theme);
        }
    }

    if let Some(dialog) = app.dialogs.active() {
        dialog.draw(frame, app, &theme);
    }

    let footer = Paragraph::new(app.status.as_str())
        .style(palette.text())
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded)
                .border_style(palette.border()),
        );

    frame.render_widget(footer, areas.footer);
}
