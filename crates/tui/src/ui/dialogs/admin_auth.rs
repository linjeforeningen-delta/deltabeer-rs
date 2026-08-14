use crate::app::dialog::AdminAuthDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

impl DialogView for AdminAuthDialog {
    fn draw(
        &self,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let area = centered(frame.area(), 56, 16);

        frame.render_widget(Clear, area);

        let card = self
            .card
            .as_deref()
            .unwrap_or("Scan admin card");

        let password = "•".repeat(
            self.password
                .as_str()
                .chars()
                .count()
        );

        let content = vec![
            Line::styled(
                "Administrator authentication",
                Style::default()
                    .fg(theme.title)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::from("Admin card"),
            Line::styled(
                format!("> {card}"),
                Style::default().fg(theme.accent),
            ),
            Line::from(""),
            Line::from("Password"),
            Line::styled(
                format!("> {password}"),
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
                Span::raw(" Authenticate    "),
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
                    .title(" Admin Authentication ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default().fg(theme.border),
                    ),
            );

        frame.render_widget(popup, area);
    }
}