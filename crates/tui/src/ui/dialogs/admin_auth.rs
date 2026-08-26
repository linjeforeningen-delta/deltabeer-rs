use crate::app::{App, dialog::AdminAuthDialog};
use crate::ui::{
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
    traits::Content,
    widgets::form::Form,
};
use ratatui::{Frame, text::Line};

impl DialogView for AdminAuthDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 16);

        let palette = theme.admin();

        let given_name = match &self.admin {
            Some(admin) => admin
                .name
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string(),
            None => t!("dialogs.admin_auth.stranger").to_string(),
        };

        let form = Form::new(0).add_hidden_field(t!("labels.password"), &self.password);

        let mut content = vec![
            Line::styled(
                t!("dialogs.admin_auth.heading").to_string(),
                theme.title_style(palette),
            ),
            Line::from(""),
            Line::from(t!("dialogs.admin_auth.welcome", name = given_name)),
        ];
        content.push(Line::from(""));
        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("Enter", t!("hints.authenticate").to_string()),
                    ("Esc", t!("hints.close").to_string()),
                ],
            ),
        ]);

        let block_title = format!(" {} ", t!("dialogs.admin_auth.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
