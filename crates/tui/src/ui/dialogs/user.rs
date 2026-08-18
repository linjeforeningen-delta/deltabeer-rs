use crate::app::App;
use crate::app::dialog::UserDialog;
use crate::ui::dialogs::DialogView;
use crate::ui::theme::THEME;
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

        let content = vec![
            Line::from(vec![
                Span::styled(self.user.name.as_str(), THEME.title_style(palette)),
                Span::raw(format!("  @{}", self.user.username)),
            ]),
            Line::from(""),
            Line::from(format!("Program      {}", self.user.program)),
            Line::from(format!("Card         {}", self.user.card_number)),
            Line::from(format!("Role         {}", self.user.role)),
            Line::from(format!("Balance      {} Δ¢", self.user.balance.0)),
            Line::from(format!("Spent        {} Δ¢", self.user.spent.0)),
            Line::from(""),
            Line::from("Amount"),
            Line::styled(format!("> {:}", self.amount), theme.selected_style(palette)),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(" Spend    "),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(" Close"),
            ]),
        ];

        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(" User Details ", palette));

        frame.render_widget(popup, area);
    }
}
