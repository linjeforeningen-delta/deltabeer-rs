pub(crate) mod layout;
pub(crate) mod theme;
pub mod pages;
mod dialogs;

use crate::app::{App, Dialog, Page};
use ratatui::{
    Frame,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(frame: &mut Frame, app: &App) {
    let theme = theme::theme(&app.auth);
    let areas = layout::app_layout(frame.area());

    let header = Block::default()
        .title(" DeltaBeer ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title_style(Style::default().fg(theme.title));

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

    if let Some(Dialog::User(state)) = &app.dialog {
        dialogs::user::draw(frame, state, &theme);
    }

    let footer = Paragraph::new(app.status.as_str())
        .style(Style::default().fg(theme.accent))
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(footer, areas.footer);
}