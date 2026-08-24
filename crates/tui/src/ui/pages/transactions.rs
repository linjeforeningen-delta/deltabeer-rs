use crate::{app::App, ui::theme::Theme};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::{Cell, Row, Table},
};

pub(crate) fn draw(frame: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let palette = if app.dialogs.active().is_some() {
        theme.dimmed()
    } else {
        theme.active(&app.auth)
    };

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
    .header(header);

    frame.render_widget(table, area);
}
