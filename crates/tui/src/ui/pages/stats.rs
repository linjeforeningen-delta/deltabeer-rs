use crate::app::page::StatsPage;
use crate::ui::theme::Palette;
use ratatui::{Frame, layout::Rect, style::Modifier, text::Line, widgets::Paragraph};

impl StatsPage {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let labels = [
            t!("stats.users").to_string(),
            t!("stats.total_balance").to_string(),
            t!("stats.total_spent").to_string(),
            t!("stats.transactions").to_string(),
        ];
        let label_width = labels
            .iter()
            .map(|label| label.chars().count())
            .max()
            .unwrap_or(0);
        if self.loading {
            frame.render_widget(
                Paragraph::new(t!("stats.loading").to_string()).style(palette.muted()),
                area,
            );
            return;
        }
        let Some(stats) = self.stats else {
            frame.render_widget(
                Paragraph::new(t!("stats.empty").to_string()).style(palette.muted()),
                area,
            );
            return;
        };
        let values = [
            stats.total_users.to_string(),
            format!("{} Δ¢", stats.total_balance.0),
            format!("{} Δ¢", stats.total_spent.0),
            stats.total_transactions.to_string(),
        ];
        let row = |label: &str, value: &str| Line::from(format!("{label:<label_width$}  {value}"));
        let content = vec![
            Line::styled(
                t!("stats.title").to_string(),
                palette.accent().add_modifier(Modifier::BOLD),
            ),
            Line::from(""),
            row(&labels[0], &values[0]),
            row(&labels[1], &values[1]),
            row(&labels[2], &values[2]),
            row(&labels[3], &values[3]),
        ];

        let widget = Paragraph::new(content).style(palette.text());

        frame.render_widget(widget, area);
    }
}
