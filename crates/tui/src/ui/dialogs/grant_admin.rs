use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::dialog::GrantAdminDialog;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

impl DialogView for GrantAdminDialog {
    fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        frame.render_widget(Clear, area);

        let password = "•".repeat(self.password.as_str().chars().count());
        let confirm_password = "•".repeat(self.confirm_password.as_str().chars().count());

        let content = vec![
            Line::from(""),
            Line::styled(
                "Create administrator credentials for this user.",
                Style::default().fg(theme.accent),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("Card       ", Style::default().fg(theme.accent)),
                Span::styled(
                    self.card.as_deref().unwrap_or("No card scanned"),
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            field_line("Password", &password, self.active_field == 0, theme),
            field_line("Confirm", &confirm_password, self.active_field == 1, theme),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "↑/↓",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Select field    "),
                Span::styled(
                    "Enter",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Grant    "),
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
                    .title(" Grant Administrator ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );

        frame.render_widget(popup, area);
    }
}

fn field_line<'a>(label: &'a str, value: &'a str, active: bool, theme: &Theme) -> Line<'a> {
    let marker = if active { "> " } else { "  " };

    Line::from(vec![
        Span::styled(
            marker,
            if active {
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.accent)
            },
        ),
        Span::styled(format!("{label:<10}"), Style::default().fg(theme.accent)),
        Span::styled(
            value,
            if active {
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.accent)
            },
        ),
    ])
}
