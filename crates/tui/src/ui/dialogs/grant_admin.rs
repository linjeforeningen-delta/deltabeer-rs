use crate::ui::reccuring::user_line;
use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::GrantAdminDialog;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for GrantAdminDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        let palette = theme.admin();

        frame.render_widget(Clear, area);

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
            Line::from(vec![
                Span::styled("↑/↓", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.select_field"))),
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.grant"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.cancel"))),
            ]),
        ]);

        let block_title = format!(" {} ", t!("dialogs.grant_admin.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
