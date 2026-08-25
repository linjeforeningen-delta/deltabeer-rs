use crate::{app::page::UsersPage, ui::theme::Palette};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::{Cell, Row, Table},
};

impl UsersPage {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
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
}