use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::dialog::RevokeAdminDialog;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

impl DialogView for RevokeAdminDialog {
    fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let area = centered(frame.area(), 54, 11);

        frame.render_widget(Clear, area);

        let content = vec![
            Line::from(""),
            Line::from("Revoke administrator privileges from this user?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("Card  ", Style::default().fg(theme.accent)),
                Span::styled(
                    self.card.as_deref().unwrap_or("No card scanned"),
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::styled(
                "This will remove administrator privileges.",
                Style::default().fg(theme.accent),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Revoke    "),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Cancel"),
            ]),
        ];

        let popup = Paragraph::new(content)
            .style(Style::default().fg(theme.accent))
            .block(
                Block::default()
                    .title(" Revoke Administrator ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );

        frame.render_widget(popup, area);
    }
}
