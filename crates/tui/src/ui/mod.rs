pub(crate) mod dialogs;
pub(crate) mod layout;
pub mod pages;
mod reccuring;
pub(crate) mod theme;
mod traits;
mod widgets;

use crate::app::{App, Page};
use crate::ui::theme::THEME;
use crate::ui::widgets::folder_tabs::FolderPageFrame;
use ratatui::layout::Margin;
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
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

    let header = Block::default()
        .title(format!(" {} ", t!("header.title")))
        .title_style(theme.title_style(palette))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(palette.border());

    frame.render_widget(header, areas.header);

    let page_frame = FolderPageFrame::new(app.page, palette);
    let body = FolderPageFrame::inner(areas.body);
    frame.render_widget(page_frame, areas.body);

    match app.page {
        Page::Home => {
            pages::home::draw(frame, body, app, &theme);
        }
        Page::Users => {
            pages::users::draw(frame, body, app, &theme);
        }
        Page::Transactions => {
            pages::transactions::draw(frame, body, app, &theme);
        }
        Page::Stats => {
            pages::stats::draw(frame, body, app, &theme);
        }
    }

    if let Some(dialog) = app.dialogs.active() {
        dialog.draw(frame, app, &theme);
    }

    let footer = Paragraph::new(app.status.as_str())
        .style(palette.text())
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_type(BorderType::Rounded)
                .border_style(palette.border()),
        );

    frame.render_widget(footer, areas.footer);
}
