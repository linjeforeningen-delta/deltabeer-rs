use crate::app::{
    App,
    dialog::{UpdateUserDialog, UpdateUserStage},
};
use crate::ui::{
    components::card_line,
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
    traits::Content,
    widgets::form::Form,
};
use ratatui::{Frame, text::Line};

impl DialogView for UpdateUserDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let palette = theme.admin();
        let block_title = format!(" {} ", t!("dialogs.update_user.title"));

        match self.stage {
            UpdateUserStage::Identify => {
                let area = centered(frame.area(), 56, 14);
                let form = Form::new(0).add_field(t!("labels.username"), &self.identifier);

                let mut content = vec![
                    Line::from(""),
                    Line::raw(t!("dialogs.update_user.scan_prompt").to_string()),
                    Line::styled(
                        t!("dialogs.update_user.or").to_string(),
                        theme.muted_style(palette),
                    ),
                    Line::from(""),
                ];

                content.extend(form.lines(theme, palette));
                content.extend([
                    Line::from(""),
                    action_hint(
                        theme,
                        palette,
                        &[
                            ("Enter", t!("hints.find").to_string()),
                            ("Esc", t!("hints.back").to_string()),
                        ],
                    ),
                ]);

                render_dialog(frame, area, &block_title, content, palette, theme);
            }

            UpdateUserStage::Edit => {
                let area = centered(frame.area(), 62, 20);
                let card_str = self
                    .replacement_card
                    .clone()
                    .or_else(|| self.user.as_ref().map(|u| u.card_number.to_string()));

                let form = Form::new(self.active_field)
                    .add_field(t!("labels.name"), &self.name)
                    .add_field(t!("labels.username"), &self.username)
                    .add_field(t!("labels.program"), &self.program)
                    .add_field(t!("labels.comments"), &self.comments);

                let editing_title = match &self.user {
                    Some(user) => t!("dialogs.update_user.editing", name = user.name),
                    None => t!("dialogs.update_user.editing_generic"),
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
                    action_hint(
                        theme,
                        palette,
                        &[
                            ("↑/↓", t!("hints.select_field").to_string()),
                            ("Enter", t!("hints.save").to_string()),
                            ("Esc", t!("hints.back").to_string()),
                        ],
                    ),
                ]);

                render_dialog(frame, area, &block_title, content, palette, theme);
            }
        }
    }
}
