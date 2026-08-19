use crate::app::App;
use crate::app::dialog::{UpdateUserDialog, UpdateUserStage};
use crate::ui::reccuring::card_line;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for UpdateUserDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let palette = theme.admin();

        match self.stage {
            UpdateUserStage::Identify => {
                let area = centered(frame.area(), 56, 14);
                frame.render_widget(Clear, area);

                let form = Form::new(0).add_field("Username", &self.identifier);

                let mut content = vec![
                    Line::from(""),
                    Line::raw("Scan a user card"),
                    Line::styled("or", theme.muted_style(palette)),
                    Line::from(""),
                ];

                content.extend(form.lines(theme, palette));
                content.extend([
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("Enter", theme.key_style(palette)),
                        Span::raw(" Find    "),
                        Span::styled("Esc", theme.key_style(palette)),
                        Span::raw(" Back"),
                    ]),
                ]);

                let popup = Paragraph::new(content)
                    .style(palette.text())
                    .block(theme.dialog_block(" Update User ", palette));

                frame.render_widget(popup, area);
            }

            UpdateUserStage::Edit => {
                let area = centered(frame.area(), 62, 20);
                frame.render_widget(Clear, area);

                let card_str = self
                    .replacement_card
                    .clone()
                    .or_else(|| self.user.as_ref().map(|u| u.card_number.to_string()));

                let form = Form::new(self.active_field)
                    .add_field("Name", &self.name)
                    .add_field("Username", &self.username)
                    .add_field("Program", &self.program)
                    .add_field("Comments", &self.comments);

                let editing_title = match &self.user {
                    Some(user) => format!("Editing {}", user.name),
                    None => "Editing User".to_string(),
                };

                let mut content = vec![
                    Line::from(""),
                    Line::raw(editing_title),
                    Line::from(""),
                    card_line(&card_str, palette),
                    Line::from(""),
                ];

                content.extend(form.lines(theme, palette));
                content.extend([
                    Line::from(""),
                    Line::from(vec![
                        Span::styled("↑/↓", theme.key_style(palette)),
                        Span::raw(" Select field    "),
                        Span::styled("Enter", theme.key_style(palette)),
                        Span::raw(" Save    "),
                        Span::styled("Esc", theme.key_style(palette)),
                        Span::raw(" Back"),
                    ]),
                ]);

                let popup = Paragraph::new(content)
                    .style(palette.text())
                    .block(theme.dialog_block(" Update User ", palette));

                frame.render_widget(popup, area);
            }
        }
    }
}
