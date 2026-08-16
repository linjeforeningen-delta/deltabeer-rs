use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::RevokeAdminDialog;
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

        let content = vec![
            Line::from(""),
            Line::from("Revoke administrator privileges from this user?"),
            Line::from(""),
            Line::from(vec![
                Span::raw("Card  "),
                Span::raw(self.card.as_deref().unwrap_or("No card scanned")),
            ]),
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
        ];

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Revoke Administrator ", palette));

        frame.render_widget(popup, area);
    }
}
