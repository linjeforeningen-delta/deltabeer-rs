use crate::app::App;
use crate::app::dialog::UserDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::{layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for UserDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 18);

        let palette = theme.active(&app.auth);

        frame.render_widget(Clear, area);

        // Pad labels to a fixed column so values line up regardless of locale.
        let detail = |label: &str, value: String| Line::from(format!("{label:<13}{value}"));

        let content = vec![
            Line::from(vec![
                Span::styled(self.user.name.as_str(), theme.title_style(palette)),
                Span::raw(format!("  @{}", self.user.username)),
            ]),
            Line::from(""),
            detail(&t!("labels.program").to_string(), self.user.program.clone()),
            detail(
                &t!("labels.card").to_string(),
                self.user.card_number.to_string(),
            ),
            detail(
                &t!("labels.role").to_string(),
                if matches!(self.user.role, crate::api::models::user::Role::Admin) {
                    t!("roles.admin").to_string()
                } else {
                    t!("roles.user").to_string()
                },
            ),
            detail(
                &t!("labels.balance").to_string(),
                format!("{} Δ¢", self.user.balance.0),
            ),
            detail(
                &t!("labels.spent").to_string(),
                format!("{} Δ¢", self.user.spent.0),
            ),
            Line::from(""),
            Line::from(t!("dialogs.user.amount").to_string()),
            Line::styled(format!("> {:}", self.amount), theme.selected_style(palette)),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.spend"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.close"))),
            ]),
        ];

        let block_title = format!(" {} ", t!("dialogs.user.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
