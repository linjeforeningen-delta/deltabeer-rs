use crate::app::App;
use crate::app::dialog::AdminSessionLoginDialog;
use crate::ui::dialogs::{DialogView, action_hint, render_dialog};
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{Frame, text::Line};

impl DialogView for AdminSessionLoginDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 16);

        let palette = theme.admin();

        let given_name = self
            .admin
            .as_ref()
            .map(|admin| admin.name.as_str())
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();

        let form = Form::new(0).add_hidden_field(t!("labels.password"), &self.password);

        let mut content = vec![
            Line::styled(
                t!("dialogs.admin_login.heading").to_string(),
                theme.title_style(palette),
            ),
            Line::from(""),
            Line::from(t!("dialogs.admin_login.start_session", name = given_name)),
            Line::from(""),
        ];
        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("Enter", t!("hints.login").to_string()),
                    ("Esc", t!("hints.back").to_string()),
                ],
            ),
        ]);

        let block_title = format!(" {} ", t!("dialogs.admin_login.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
