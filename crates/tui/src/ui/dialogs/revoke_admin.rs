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

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        let prompt = match &self.user {
            Some(user) => format!("Revoke administrator privileges from {}?", user.name),
            None => "Revoke administrator privileges from this user?".to_string(),
        };

        let mut content = vec![
            Line::from(""),
            Line::from(prompt),
            Line::from(""),
        ];

        content.push(user_line(&self.user, palette));
        content.extend([
            Line::from(""),
            Line::styled(
                "This will remove administrator privileges.",
                theme.selected_style(palette),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Revoke    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Cancel"),
            ]),
        ]);

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Revoke Administrator ", palette));

        frame.render_widget(popup, area);
    }
}
