use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::dialog::TopUpDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::{layout::centered, theme::Theme};

impl DialogView for TopUpDialog {
    fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        frame.render_widget(Clear, area);

        let content = vec![
            Line::from(vec![
                Span::styled(
                    &self.user.name,
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("  @{}", self.user.username),
                    Style::default().fg(theme.accent),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Current balance  ", Style::default().fg(theme.accent)),
                Span::styled(
                    format!("{} Δ¢", self.user.balance.0),
                    Style::default().fg(theme.accent),
                ),
            ]),
            Line::from(""),
            Line::styled("Top-up amount", Style::default().fg(theme.accent)),
            Line::styled(
                format!("> {}", self.amount.as_str()),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            Line::styled(
                "Enter Top up    Esc Back",
                Style::default().fg(theme.accent),
            ),
        ];

        let popup = Paragraph::new(content).block(
            Block::default()
                .title(" Top Up ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        );

        frame.render_widget(popup, area);
    }
}
