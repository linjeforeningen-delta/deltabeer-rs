use crate::ui::{
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
};

use crate::app::App;
use crate::app::dialog::RevokeAdminDialog;
use crate::ui::components::user_line;
use ratatui::{Frame, text::Line};

impl DialogView for RevokeAdminDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 54, 11);

        let palette = theme.admin();

        let prompt = match &self.user {
            Some(user) => t!("dialogs.revoke_admin.prompt", name = user.name),
            None => t!("dialogs.revoke_admin.prompt_generic"),
        };

        let mut content = vec![Line::from(""), Line::from(prompt), Line::from("")];

        content.push(user_line(&self.user, palette));
        content.extend([
            Line::from(""),
            Line::styled(
                t!("dialogs.revoke_admin.warning").to_string(),
                theme.selected_style(palette),
            ),
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("Enter", t!("hints.revoke").to_string()),
                    ("Esc", t!("hints.cancel").to_string()),
                ],
            ),
        ]);

        let block_title = format!(" {} ", t!("dialogs.revoke_admin.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
