use crate::app::dialog::UserDialog;
use crate::ui::dialogs::DialogView;
use crate::
ui::{layout::centered, theme::Theme};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

impl DialogView for UserDialog {
    fn draw(
        &self,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let area = centered(frame.area(), 56, 18);

        frame.render_widget(Clear, area);

        let content = vec![
            Line::from(vec![
                Span::styled(
                    self.user.name.as_str(),
                    Style::default()
                        .fg(theme.title)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!("  @{}", self.user.username)),
            ]),
            Line::from(""),
            Line::from(format!(
                "Program      {}",
                self.user.program
            )),
            Line::from(format!(
                "Card         {}",
                self.user.card_number
            )),
            Line::from(format!(
                "Role         {}",
                self.user.role
            )),
            Line::from(format!(
                "Balance      {} Δ¢",
                self.user.balance.0
            )),
            Line::from(format!(
                "Spent        {} Δ¢",
                self.user.spent.0
            )),
            Line::from(""),
            Line::from("Amount"),
            Line::styled(
                format!("> {:}", self.amount),
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
                Span::raw(" Spend    "),
                Span::styled(
                    "T",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Top up    "),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Close"),
            ]),
        ];

        let popup = Paragraph::new(content)
            .style(Style::default().fg(theme.accent))
            .block(
                Block::default()
                    .title(" User ")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme.border)),
            );

        frame.render_widget(popup, area);
    }
}