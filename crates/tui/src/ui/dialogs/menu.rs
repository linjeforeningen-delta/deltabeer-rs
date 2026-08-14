use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::app::dialog::MenuDialog;
use crate::
ui::{
    dialogs::DialogView,
    layout::centered,
    theme::Theme,
};

impl DialogView for MenuDialog {
    fn draw(
        &self,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let content_width = self
            .options
            .iter()
            .map(|option| {
                // "[X] Option name"
                option.name.len() + 4
            })
            .max()
            .unwrap_or(0);

        let title_width = self.title.len() + 4;

        let width = content_width
            .max(title_width)
            .max(24)
            .saturating_add(4) as u16;

        // 2 border rows
        // + one row per option
        // + 2 rows padding/footer
        let height = self
            .options
            .len()
            .saturating_add(4) as u16;
        let popup_area = centered(
            Rect::new(0, 0, width, height),
            width,
            height,
        );

        frame.render_widget(Clear, popup_area);

        let mut lines = Vec::with_capacity(
            self.options.len() + 1
        );

        for option in &self.options {
            lines.push(
                Line::from(vec![
                    Span::styled(
                        format!("[{}]", option.key.to_ascii_uppercase()),
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" {}", option.name)),
                ])
            );
        }

        lines.push(Line::from(""));

        lines.push(
            Line::from(vec![
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" Back"),
            ])
        );

        let popup = Paragraph::new(lines)
            .style(
                Style::default()
                    .fg(theme.accent)
            )
            .block(
                Block::default()
                    .title(format!(" {} ", self.title))
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_style(
                        Style::default()
                            .fg(theme.border)
                    ),
            );

        frame.render_widget(popup, popup_area);
    }
}