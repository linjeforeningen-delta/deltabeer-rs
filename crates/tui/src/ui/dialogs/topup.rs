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

        let palette = theme.admin();

        frame.render_widget(Clear, area);

        let mut content = vec![user_line(&self.user, palette)];

        content.extend([
            Line::from(""),
            Line::raw(t!("dialogs.topup.amount_prompt").to_string()),
            Line::styled(
                format!("> {}", self.amount.as_str()),
                theme.selected_style(palette),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.top_up"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.close"))),
            ]),
        ]);

        let block_title = format!(" {} ", t!("dialogs.topup.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
