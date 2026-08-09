use crate::{
    app::App,
    ui::theme::Theme,
};
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(
    frame: &mut Frame,
    area: Rect,
    _app: &App,
    theme: &Theme,
) {
    let content = vec![
        Line::from("Welcome to DeltaBeer"),
        Line::from(""),
        Line::from("Scan a card at any time to open a user."),
        Line::from(""),
        Line::from("1 Home"),
        Line::from("2 Users"),
        Line::from("3 Transactions"),
        Line::from("4 Stats"),
    ];

    let widget = Paragraph::new(content)
        .style(Style::default().fg(theme.accent))
        .block(
            Block::default()
                .title(" Home ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        );

    frame.render_widget(widget, area);
}