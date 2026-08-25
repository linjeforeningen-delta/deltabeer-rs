pub(crate) mod dialogs;
pub(crate) mod layout;
pub mod pages;
mod reccuring;
pub(crate) mod theme;
mod traits;
mod widgets;

use crate::app::App;
use crate::ui::theme::THEME;
use crate::ui::widgets::folder_tabs::FolderPageFrame;
use ratatui::layout::Margin;
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(frame: &mut Frame, app: &App) {
    let theme = THEME;
    let palette = theme.active(&app.auth);
    let area = frame.area().inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let areas = layout::app_layout(area);

    let page_frame = FolderPageFrame::new(app.page, palette);
    let body = FolderPageFrame::inner(areas.body);
    frame.render_widget(page_frame, areas.body);

    app.page.draw(frame, body, app, &theme);

    if let Some(dialog) = app.dialogs.active() {
        dialog.draw(frame, app, &theme);
    }

    let footer = Paragraph::new(Line::from(format!(
        "{}  |  {}",
        app.status,
        t!("hints.change_language")
    )))
        .style(palette.text())
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded)
                .border_style(palette.border()),
        );

    frame.render_widget(footer, areas.footer);
}
