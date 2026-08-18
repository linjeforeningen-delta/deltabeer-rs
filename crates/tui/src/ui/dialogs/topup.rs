use crate::app::App;
use crate::app::dialog::TopUpDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::reccuring::user_line;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{
    prelude::*,
    widgets::{Clear, Paragraph},
};

impl DialogView for TopUpDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        let mut content = vec![user_line(&self.user, palette)];

        content.extend([
            Line::from(""),
            Line::raw("Top-up amount"),
            Line::styled(
                format!("> {}", self.amount.as_str()),
                theme.selected_style(palette),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Top up    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Close"),
            ]),
        ]);

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Top Up ", palette));

        frame.render_widget(popup, area);
    }
}
