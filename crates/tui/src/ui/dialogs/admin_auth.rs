use crate::app::App;
use crate::app::dialog::AdminAuthDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for AdminAuthDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 16);

        let palette = theme.admin();

        frame.render_widget(Clear, area);

        let card = self.card.as_deref().unwrap_or("Scan admin card");

        let password = "•".repeat(self.password.as_str().chars().count());

        let form = Form::new(0)
            .add_hidden_field("Password", &self.password);

        let mut content = vec![
            Line::styled("Administrator authentication", theme.title_style(palette)),
            Line::from(""),
            Line::from("Admin card"),
            Line::raw(format!("> {card}")),
            Line::from(""), ];
        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Authenticate    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Close"),
            ]),
        ]);

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Admin Authentication ", palette));

        frame.render_widget(popup, area);
    }
}
