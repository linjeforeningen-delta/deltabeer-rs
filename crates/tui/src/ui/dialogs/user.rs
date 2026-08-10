use crate::{
    app::UserDialogState,
    ui::{layout::centered, theme::Theme},
};
use ratatui::{
    Frame,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
};

pub(crate) fn draw(
    frame: &mut Frame,
    state: &UserDialogState,
    theme: &Theme,
) {
    let area = centered(frame.area(), 50, 14);

    frame.render_widget(Clear, area);

    let content = vec![
        Line::from("Card scanned"),
        Line::from(""),
        Line::from(format!("Card: {}", state.user.card_number)),
        Line::from(""),
        Line::from("Amount:"),
        Line::from(state.amount.as_str()),
        Line::from(""),
        Line::from("Enter Spend    Esc Close"),
    ];

    let popup = Paragraph::new(content)
        .style(Style::default().fg(theme.accent))
        .block(
            Block::default()
                .title(" User ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        );

    frame.render_widget(popup, area);
}