use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::MakeUserDialog;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for MakeUserDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 62, 20);

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        let card = self.card.as_deref().unwrap_or("Scan card");

        let form = Form::new(self.active_field)
            .add_field("Name", &self.name)
            .add_field("Username", &self.username)
            .add_field("Program", &self.program)
            .add_field("Birthdate", &self.birthdate);

        let mut content = form.lines(theme, palette);
        content.extend(
            [
                Line::from(""),
                Line::from(vec![Span::raw("Card       "), Span::raw(card)]),
                Line::from(""),
                Line::styled("Birthdate format: YYYY-MM-DD", theme.muted_style(palette)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("↑/↓", theme.selected_style(palette)),
                    Span::raw(" Select field    "),
                    Span::styled("Enter", theme.key_style(palette)),
                    Span::raw(" Create    "),
                    Span::styled("Esc", theme.key_style(palette)),
                    Span::raw(" Back"),
                ]),
            ]
        );

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Create User ", palette));

        frame.render_widget(popup, area);
    }
}
