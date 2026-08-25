use crate::{app::page::StatsPage, ui::theme::Palette};
use ratatui::{Frame, layout::Rect, style::Modifier, text::Line, widgets::Paragraph};


impl StatsPage {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        // Pad labels to a fixed column so the placeholder values line up regardless
        // of the locale's word lengths.
        let row = |label: &str| Line::from(format!("{label:<16}--"));
        let content = vec![
            Line::styled(
                t!("stats.title").to_string(),
                palette.accent().add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            row(&t!("stats.users").to_string()),
            row(&t!("stats.total_balance").to_string()),
            row(&t!("stats.total_spent").to_string()),
            row(&t!("stats.transactions").to_string()),
        ];

        let widget = Paragraph::new(content).style(palette.text());

        frame.render_widget(widget, area);
    }
}
