use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::dialog::MakeUserDialog;
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

impl DialogView for MakeUserDialog {
    fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let area = centered(frame.area(), 62, 20);

        frame.render_widget(Clear, area);

        let card = self.card.as_deref().unwrap_or("Scan card");

        let content = vec![
            field_line("Name", self.name.as_str(), self.active_field == 0, theme),
            field_line(
                "Username",
                self.username.as_str(),
                self.active_field == 1,
                theme,
            ),
            field_line(
                "Program",
                self.program.as_str(),
                self.active_field == 2,
                theme,
            ),
            field_line(
                "Birthdate",
                self.birthdate.as_str(),
                self.active_field == 3,
                theme,
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("Card       ", Style::default().fg(theme.accent)),
                Span::styled(card, Style::default().fg(theme.accent)),
            ]),
            Line::from(""),
            Line::styled(
                "Birthdate format: YYYY-MM-DD",
                Style::default().fg(theme.accent),
            ),
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
                Span::raw(" Create    "),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Back"),
            ]),
        ];

        let popup = Paragraph::new(content)
            .style(Style::default().fg(theme.accent))
            .block(
                Block::default()
                    .title(" Create User ")
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
