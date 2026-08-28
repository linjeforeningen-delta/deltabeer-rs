use crate::app::{AboutDialog, App, metadata};
use crate::ui::{
    dialogs::{DialogView, action_hint, render_dialog},
    layout::centered,
    theme::Theme,
};
use ratatui::{
    Frame,
    text::{Line, Span},
};

impl DialogView for AboutDialog {
    fn draw(&self, frame: &mut Frame, _app: &App, theme: &Theme) {
        let palette = theme.normal();
        let lines = vec![
            Line::from(Span::styled(
                t!("about.application_name").to_string(),
                theme.title_style(palette),
            )),
            Line::from(t!("about.version", version = env!("CARGO_PKG_VERSION"))),
            Line::from(t!(
                "about.first_release",
                date = metadata::FIRST_RELEASE_DATE
            )),
            Line::from(t!("about.updated", date = metadata::UPDATED_DATE)),
            Line::from(""),
            action_hint(
                theme,
                palette,
                &[("Esc/Enter", t!("hints.close").to_string())],
            ),
        ];

        let area = centered(frame.area(), 42, lines.len() as u16 + 4);
        render_dialog(
            frame,
            area,
            &format!(" {} ", t!("about.title")),
            lines,
            palette,
            theme,
        );
    }
}
