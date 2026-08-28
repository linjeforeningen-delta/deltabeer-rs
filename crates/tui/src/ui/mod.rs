mod components;
pub(crate) mod dialogs;
pub(crate) mod layout;
pub(crate) mod localization;
pub(crate) mod pages;
pub(crate) mod theme;
mod traits;
mod widgets;

use crate::app::App;
use crate::ui::{
    localization::localize_status, theme::THEME, widgets::folder_tabs::FolderPageFrame,
};
use ratatui::layout::Margin;
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub(crate) fn draw(frame: &mut Frame, app: &mut App) {
    let theme = THEME;
    let page_palette = pages::page_palette(app, &theme);
    let active_palette = theme.active(&app.auth);
    let area = frame.area().inner(Margin {
        horizontal: 2,
        vertical: 1,
    });

    let areas = layout::app_layout(area);

    let page_frame = FolderPageFrame::new(app.page.id(), page_palette);
    let body = FolderPageFrame::inner(areas.body);
    frame.render_widget(page_frame, areas.body);

    app.page.draw(frame, body, page_palette);

    if let Some(dialog) = app.dialogs.active() {
        dialog.draw(frame, app, &theme);
    }

    let footer = Paragraph::new(Line::from(format!(
        "{}  |  {}",
        localize_status(&app.status),
        t!("hints.change_language")
    )))
    .style(active_palette.text())
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Rounded)
            .border_style(active_palette.border()),
    );

    frame.render_widget(footer, areas.footer);
}
