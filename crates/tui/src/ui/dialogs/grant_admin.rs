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

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        let form = Form::new(self.active_field)
            .add_hidden_field("Password", &self.password)
            .add_hidden_field("Confirm", &self.confirm_password);

        let mut content = vec![
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
            Line::from("")];

        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            Line::from(vec![
                Span::styled("↑/↓", theme.key_style(palette)),
                Span::raw(" Select field    "),
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Grant    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Cancel"),
            ]),
        ]);

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Grant Administrator ", palette));

        frame.render_widget(popup, area);
    }
}
