use crate::ui::components::user_line;
use crate::ui::{
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
};

use crate::app::App;
use crate::app::dialog::GrantAdminDialog;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use ratatui::{Frame, text::Line};

impl DialogView for GrantAdminDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        let palette = theme.admin();

        let form = Form::new(self.active_field)
            .add_hidden_field(t!("labels.password"), &self.password)
            .add_hidden_field(t!("labels.confirm"), &self.confirm_password);

        let prompt = match &self.user {
            Some(user) => t!("dialogs.grant_admin.prompt", name = user.name),
            None => t!("dialogs.grant_admin.prompt_generic"),
        };

        let mut content = vec![
            Line::from(""),
            Line::raw(prompt),
            Line::from(""),
            user_line(&self.user, palette),
            Line::from(""),
        ];

        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("↑/↓", t!("hints.select_field").to_string()),
                    ("Enter", t!("hints.grant").to_string()),
                    ("Esc", t!("hints.cancel").to_string()),
                ],
            ),
        ]);

        let block_title = format!(" {} ", t!("dialogs.grant_admin.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
