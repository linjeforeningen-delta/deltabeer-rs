use crate::ui::{dialogs::DialogView, layout::centered, theme::Theme};

use crate::app::App;
use crate::app::dialog::MakeUserDialog;
use crate::ui::reccuring::card_line;
use crate::ui::traits::Content;
use crate::ui::widgets::form::Form;
use ratatui::{
    Frame,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

impl DialogView for MakeUserDialog {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme) {
        let area = centered(frame.area(), 62, 20);

        let palette = theme.admin();

        frame.render_widget(Clear, area);

        let form = Form::new(self.active_field)
            .add_field(t!("labels.name"), &self.name)
            .add_field(t!("labels.username"), &self.username)
            .add_field(t!("labels.program"), &self.program)
            .add_field(t!("labels.birthdate"), &self.birthdate);

        let mut content = form.lines(theme, palette);
        content.push(Line::from(""));
        content.push(card_line(&self.card, palette));
        content.extend([
            Line::from(""),
            Line::styled(
                t!("dialogs.make_user.birthdate_format").to_string(),
                theme.muted_style(palette),
            ),
            Line::from(""),
            Line::from(vec![
                Span::styled("↑/↓", theme.selected_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.select_field"))),
                Span::styled("Enter", theme.key_style(palette)),
                Span::raw(format!(" {}    ", t!("hints.create"))),
                Span::styled("Esc", theme.key_style(palette)),
                Span::raw(format!(" {}", t!("hints.back"))),
            ]),
        ]);

        let block_title = format!(" {} ", t!("dialogs.make_user.title"));
        let popup = Paragraph::new(content)
            .style(palette.text())
            .block(theme.dialog_block(&block_title, palette));

        frame.render_widget(popup, area);
    }
}
