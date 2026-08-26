use crate::app::{App, dialog::UserDialog};
use crate::model::Role;
use crate::ui::{
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
};
use ratatui::{
    Frame,
    text::{Line, Span},
};

impl DialogView for UserDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 56, 18);

        let palette = theme.active(&app.auth);

        let labels = [
            t!("labels.program").to_string(),
            t!("labels.card").to_string(),
            t!("labels.role").to_string(),
            t!("labels.balance").to_string(),
            t!("labels.spent").to_string(),
        ];
        let label_width = labels
            .iter()
            .map(|label| label.chars().count())
            .max()
            .unwrap_or(0);
        let detail =
            |label: &str, value: String| Line::from(format!("{label:<label_width$} {value}"));

        let content = vec![
            Line::from(vec![
                Span::styled(self.user.name.as_str(), theme.title_style(palette)),
                Span::raw(format!("  @{}", self.user.username)),
            ]),
            Line::from(""),
            detail(&labels[0], self.user.program.clone()),
            detail(&labels[1], self.user.card_number.to_string()),
            detail(
                &labels[2],
                if matches!(self.user.role, Role::Admin) {
                    t!("roles.admin").to_string()
                } else {
                    t!("roles.user").to_string()
                },
            ),
            detail(&labels[3], format!("{} Δ¢", self.user.balance.0)),
            detail(&labels[4], format!("{} Δ¢", self.user.spent.0)),
            Line::from(""),
            Line::from(t!("dialogs.user.amount").to_string()),
            Line::styled(format!("> {:}", self.amount), theme.selected_style(palette)),
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[
                    ("Enter", t!("hints.spend").to_string()),
                    ("Esc", t!("hints.close").to_string()),
                ],
            ),
        ];

        let block_title = format!(" {} ", t!("dialogs.user.title"));
        render_dialog(frame, area, &block_title, content, palette, theme);
    }
}
