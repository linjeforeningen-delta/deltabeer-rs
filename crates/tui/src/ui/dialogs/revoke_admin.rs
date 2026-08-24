use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::RevokeAdminDialog;
use crate::ui::reccuring::user_line;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for RevokeAdminDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 54, 11);

        let palette = theme.admin();

        frame.render_widget(Clear, area);

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
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.revoke"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.cancel"))),
            ]),
        ]);

        let block_title = format!(" {} ", t!("dialogs.revoke_admin.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
