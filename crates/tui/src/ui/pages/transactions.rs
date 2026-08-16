use crate::{app::App, ui::theme::Theme};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::{Cell, Row, Table},
};

pub(crate) fn draw(frame: &mut Frame, area: Rect, _app: &App, theme: &Theme) {
    let palette = theme.active(&_app.auth);

    let header = Row::new([
        Cell::from("Time"),
        Cell::from("User"),
        Cell::from("Type"),
        Cell::from("Amount"),
        Cell::from("Source"),
    ])
    .style(palette.accent().add_modifier(Modifier::BOLD));

    let rows = Vec::<Row>::new();

    let table = Table::new(
        rows,
        [
            Constraint::Length(20),
            Constraint::Percentage(30),
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(15),
        ],
    )
    .header(header)
    .block(theme.page_block(" Transactions ", palette));

    frame.render_widget(table, area);
}
