use crate::{app::App, ui::theme::Theme};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(frame: &mut Frame, area: Rect, _app: &App, theme: &Theme) {
    let content = vec![
        Line::styled(
            "System statistics",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
        Line::from("Users           --"),
        Line::from("Total balance   --"),
        Line::from("Total spent     --"),
        Line::from("Transactions    --"),
    ];

    let widget = Paragraph::new(content)
        .style(Style::default().fg(theme.accent))
        .block(
            Block::default()
                .title(" Stats ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        );

    frame.render_widget(widget, area);
}
