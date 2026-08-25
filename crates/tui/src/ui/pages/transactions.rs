use crate::{app::App, app::page::TransactionsPage, ui::theme::Theme};
use ratatui::{
    Frame,
    layout::{Constraint, Rect},
    style::Modifier,
    widgets::{Cell, Row, Table},
};

impl TransactionsPage {
    pub(crate) fn draw(
        &self,
        frame: &mut Frame,
        area: Rect,
        app: &App,
        theme: &Theme,
    ) {
        let palette = if app.dialogs.active().is_some() {
            theme.dimmed()
        } else {
            theme.active(&app.auth)
        };

        let header = Row::new([
            Cell::from(t!("tx_table.time").to_string()),
            Cell::from(t!("tx_table.user").to_string()),
            Cell::from(t!("tx_table.kind").to_string()),
            Cell::from(t!("tx_table.amount").to_string()),
            Cell::from(t!("tx_table.source").to_string()),
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
}
