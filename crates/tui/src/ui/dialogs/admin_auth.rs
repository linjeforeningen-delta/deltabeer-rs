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

        let given_name = match &self.admin {
            Some(admin) => admin
                .name
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string(),
            None => t!("dialogs.admin_auth.stranger").to_string(),
        };

        let form = Form::new(0).add_hidden_field(t!("labels.password"), &self.password);

        let mut content = vec![
            Line::styled(
                t!("dialogs.admin_auth.heading").to_string(),
                theme.title_style(palette),
            ),
            Line::from(""),
            Line::from(t!("dialogs.admin_auth.welcome", name = given_name)),
        ];
        content.push(Line::from(""));
        content.extend(form.lines(theme, palette));
        content.extend([
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.authenticate"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.close"))),
            ]),
        ]);

        let block_title = format!(" {} ", t!("dialogs.admin_auth.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
