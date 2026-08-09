pub(crate) mod layout;
pub(crate) mod theme;

use crate::app::App;
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

    frame.render_widget(
        Paragraph::new("Main content"),
        areas.body,
    );

    let footer = Paragraph::new(app.status.as_str())
        .style(Style::default().fg(theme.accent))
        .block(Block::default().borders(Borders::TOP));

    frame.render_widget(footer, areas.footer);
}