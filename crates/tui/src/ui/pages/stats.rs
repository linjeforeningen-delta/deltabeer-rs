use crate::{app::App, ui::theme::Theme};
use ratatui::{Frame, layout::Rect, style::Modifier, text::Line, widgets::Paragraph};

pub(crate) fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let palette = if app.dialogs.active().is_some() {
        theme.dimmed()
    } else {
        theme.active(&app.auth)
    };

    let content = vec![
        Line::styled(
            "System statistics",
            palette.accent().add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from("Users           --"),
        Line::from("Total balance   --"),
        Line::from("Total spent     --"),
        Line::from("Transactions    --"),
    ];

    let widget = Paragraph::new(content).style(palette.text());

    frame.render_widget(widget, area);
}
