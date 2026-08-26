use crate::app::App;
use crate::app::dialog::TopUpDialog;
use crate::ui::components::user_line;
use crate::ui::dialogs::{DialogView, action_hint, render_dialog};
use crate::ui::{layout::centered, theme::Theme};
use ratatui::prelude::*;

impl DialogView for TopUpDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        let palette = theme.admin();

        let mut content = vec![user_line(&self.user, palette)];

        content.extend([
            Line::from(""),
            Line::raw(t!("dialogs.topup.amount_prompt").to_string()),
            Line::styled(
                format!("> {}", self.amount.as_str()),
                theme.selected_style(palette),
            ),
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("Enter", t!("hints.top_up").to_string()),
                    ("Esc", t!("hints.close").to_string()),
                ],
            ),
        ]);

        let block_title = format!(" {} ", t!("dialogs.topup.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
