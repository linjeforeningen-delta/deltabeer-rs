use crate::app::App;
use crate::app::dialog::MenuDialog;
use crate::app::dialog::menu::{MenuLabel, MenuTitle};
use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for MenuDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let palette = if self.is_admin {
            theme.admin()
        } else {
            theme.active(&app.auth)
        };

        let title = match self.title {
            MenuTitle::Admin => t!("menu.admin").to_string(),
            MenuTitle::Application => t!("menu.application").to_string(),
            MenuTitle::Language => t!("menu.language").to_string(),
        };
        let content_width = self
            .options
            .iter()
            .map(|option| {
                // "[X] Option name"
                option_label(option.label).len() + 4
            })
            .max()
            .unwrap_or(0);

        let title_width = title.len() + 4;

        let width = content_width.max(title_width).max(24).saturating_add(4) as u16;

        // 2 border rows
        // + one row per option
        // + 2 rows padding/footer
        let height = self.options.len().saturating_add(4) as u16;
        let popup_area = centered(frame.area(), width, height);

        frame.render_widget(Clear, popup_area);

        let mut lines = Vec::with_capacity(self.options.len() + 1);

        for option in &self.options {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("[{}]", option.key.to_ascii_uppercase()),
                    theme.key_style(palette),
                ),
                Span::raw(format!(" {}", option_label(option.label))),
            ]));
        }

        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Esc", theme.key_style(palette)),
            Span::raw(format!(" {}", t!("hints.back"))),
        ]));

        let title = format!(" {} ", title);

        let popup = Paragraph::new(lines)
            .style(palette.text())
            .block(theme.dialog_block(&*title, palette));

        frame.render_widget(popup, popup_area);
    }
}

fn option_label(label: MenuLabel) -> String {
    match label {
        MenuLabel::ChangeLanguage => t!("menu.change_language").to_string(),
        MenuLabel::Quit => t!("menu.quit").to_string(),
        MenuLabel::English => t!("languages.en").to_string(),
        MenuLabel::NorwegianBokmaal => t!("languages.nb").to_string(),
        MenuLabel::TopUp => t!("menu.top_up").to_string(),
        MenuLabel::MakeUser => t!("menu.make_user").to_string(),
        MenuLabel::UpdateUser => t!("menu.update_user").to_string(),
        MenuLabel::GrantAdmin => t!("menu.grant_admin").to_string(),
        MenuLabel::RevokeAdmin => t!("menu.revoke_admin").to_string(),
        MenuLabel::Login => t!("menu.login").to_string(),
        MenuLabel::Logout => t!("menu.logout").to_string(),
    }
}
