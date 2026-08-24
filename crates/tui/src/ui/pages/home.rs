use crate::{app::App, ui::theme::Theme};
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

pub(crate) fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let palette = if app.dialogs.active().is_some() {
        theme.dimmed()
    } else {
        theme.active(&app.auth)
    };

    let content = vec![
        Line::from(t!("home.welcome").to_string()),
        Line::from(""),
        Line::from(t!("home.scan_hint").to_string()),
        Line::from(""),
    ];

    let widget = Paragraph::new(content).style(palette.text());

    frame.render_widget(widget, area);
}
