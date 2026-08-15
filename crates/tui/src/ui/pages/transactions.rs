use crate::{app::App, ui::theme::Theme};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

pub(crate) fn draw(frame: &mut Frame, area: Rect, _app: &App, theme: &Theme) {
    let header = Row::new([
        Cell::from("Time"),
        Cell::from("User"),
        Cell::from("Type"),
        Cell::from("Amount"),
        Cell::from("Source"),
    ])
    .style(
        Style::default()
            .fg(theme.accent)
            .add_modifier(Modifier::BOLD),
    );

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
    .block(
        Block::default()
            .title(" Transactions ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border)),
    );

    frame.render_widget(table, area);
}
