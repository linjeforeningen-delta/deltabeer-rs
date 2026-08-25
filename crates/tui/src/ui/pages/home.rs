use crate::{app::page::HomePage, ui::theme::Palette};
use ratatui::{Frame, layout::Rect, text::Line, widgets::Paragraph};

impl HomePage {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let content = vec![
            Line::from(t!("home.welcome").to_string()),
            Line::from(""),
            Line::from(t!("home.scan_hint").to_string()),
            Line::from(""),
        ];

        let widget = Paragraph::new(content).style(palette.text());

        frame.render_widget(widget, area);
    }
}
