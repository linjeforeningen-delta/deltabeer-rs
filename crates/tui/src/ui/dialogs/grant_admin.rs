use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::GrantAdminDialog;
use crate::ui::helpers::field_line;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for GrantAdminDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 14);

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        let password = "•".repeat(self.password.as_str().chars().count());
        let confirm_password = "•".repeat(self.confirm_password.as_str().chars().count());

        let content = vec![
            Line::from(""),
            Line::raw("Create administrator credentials for this user."),
            Line::from(""),
            Line::from(vec![
                Span::raw("Card       "),
                Span::styled(
                    self.card.as_deref().unwrap_or("No card scanned"),
                    theme.selected_style(palette),
                ),
            ]),
            Line::from(""),
            field_line(
                "Password",
                &password,
                self.active_field == 0,
                theme,
                palette,
            ),
            field_line(
                "Confirm",
                &confirm_password,
                self.active_field == 1,
                theme,
                palette,
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("↑/↓", theme.key_style(palette)),
                Span::raw(" Select field    "),
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Grant    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Cancel"),
            ]),
        ];

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Grant Administrator ", palette));

        frame.render_widget(popup, area);
    }
}
