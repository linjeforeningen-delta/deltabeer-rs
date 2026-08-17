use crate::{app::App, ui::theme::Theme};
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

pub(crate) fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let palette = if app.dialogs.active().is_some() {
        theme.dimmed()
    } else {
        theme.active(&app.auth)
    };

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
        .style(palette.text())
        .block(theme.page_block(" Home ", palette));

    frame.render_widget(widget, area);
}
