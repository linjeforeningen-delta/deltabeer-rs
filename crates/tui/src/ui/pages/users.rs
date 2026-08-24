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
        Cell::from(t!("users_table.name").to_string()),
        Cell::from(t!("users_table.username").to_string()),
        Cell::from(t!("users_table.card").to_string()),
        Cell::from(t!("users_table.role").to_string()),
        Cell::from(t!("users_table.balance").to_string()),
    ])
    .style(palette.accent().add_modifier(Modifier::BOLD));

    // Temporary rows until API data is wired in.
    let rows = Vec::<Row>::new();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(header);

    frame.render_widget(table, area);
}
