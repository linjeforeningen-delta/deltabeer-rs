use crate::app::App;
use crate::app::dialog::MenuDialog;
use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for MenuDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let palette = if self.is_admin {
            theme.admin()
        } else {
            theme.active(&app.auth)
        };

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

        let width = content_width.max(title_width).max(24).saturating_add(4) as u16;

        // 2 border rows
        // + one row per option
        // + 2 rows padding/footer
        let height = self.options.len().saturating_add(4) as u16;
        let popup_area = centered(frame.area(), width, height);

        frame.render_widget(Clear, popup_area);

        let mut lines = Vec::with_capacity(self.options.len() + 1);

        for option in &self.options {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{}]", option.key.to_ascii_uppercase()),
                    theme.key_style(palette),
                ),
                Span::raw(format!(" {}", option.name)),
            ]));
        }

        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Esc", theme.key_style(palette)),
            Span::raw(" Back"),
        ]));

        let title = format!(" {} ", self.title);

        let popup = Paragraph::new(lines)
            .style(palette.text())
            .block(theme.dialog_block(&*title, palette));

        frame.render_widget(popup, popup_area);
    }
}
