use crate::app::App;
use crate::app::dialog::AdminSessionLoginDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for AdminSessionLoginDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 16);

        let palette = theme.admin();

        frame.render_widget(Clear, area);

        let given_name = self
            .admin
            .as_ref()
            .map(|admin| admin.name.as_str())
            .unwrap_or("")
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();

        let form = Form::new(0).add_hidden_field("Password", &self.password);

        let mut content = vec![
            Line::styled("Admin Login", theme.title_style(palette)),
            Line::from(""),
            Line::from(format!("Start session as {}?", given_name)),
            Line::from(""),
        ];
        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Login    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Back"),
            ]),
        ]);

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" Admin Login ", palette));

        frame.render_widget(popup, area);
    }
}
