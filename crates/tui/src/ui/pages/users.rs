use crate::app::page::{SortOrder, UserSort, UsersPage};
use crate::ui::theme::Palette;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

impl UsersPage {
    pub(crate) fn draw(&self, frame: &mut Frame, area: Rect, palette: Palette) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(area);
        let search = Paragraph::new(self.search.as_str())
            .style(if self.search_active {
                palette.text()
            } else {
                palette.muted()
            })
            .block(Block::default().title(" Search (/) ").borders(Borders::ALL));
        frame.render_widget(search, chunks[0]);
        if self.loading {
            frame.render_widget(
                Paragraph::new("Loading users...").style(palette.muted()),
                chunks[1],
            );
            return;
        }
        let visible = self.visible_users();
        if self.users.is_empty() {
            frame.render_widget(
                Paragraph::new("No users found.").style(palette.muted()),
                chunks[1],
            );
            return;
        }
        if visible.is_empty() {
            frame.render_widget(
                Paragraph::new("No users match the search.").style(palette.muted()),
                chunks[1],
            );
            return;
        }

        let header = Row::new([
            self.header("Name", UserSort::Name),
            self.header("Username", UserSort::Username),
            self.header("Program", UserSort::Program),
            self.header("Card", UserSort::Card),
            self.header("Role", UserSort::Role),
            self.header("Birthdate", UserSort::Birthdate),
            self.header("Balance", UserSort::Balance),
            self.header("Spent", UserSort::Spent),
        ])
        .style(palette.accent().add_modifier(Modifier::BOLD));
        let rows = visible.iter().map(|user| {
            let row = Row::new([
                Cell::from(user.name.clone()),
                Cell::from(user.username.clone()),
                Cell::from(user.program.clone()),
                Cell::from(user.card_number.to_string()),
                Cell::from(user.role.to_string()),
                Cell::from(user.birthdate.to_string()),
                Cell::from(format!("{} Δ¢", user.balance.0)),
                Cell::from(format!("{} Δ¢", user.spent.0)),
            ]);
            if Some(user.id) == self.selected_user_id {
                row.style(palette.accent().add_modifier(Modifier::BOLD))
            } else {
                row
            }
        });
        let table = Table::new(
            rows,
            [
                Constraint::Percentage(17),
                Constraint::Percentage(14),
                Constraint::Percentage(15),
                Constraint::Length(10),
                Constraint::Length(9),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
            ],
        )
        .header(header);
        frame.render_widget(table, chunks[1]);
    }

    fn header(&self, label: &str, field: UserSort) -> Cell<'static> {
        let indicator = if self.sort_field == field {
            match self.sort_order {
                SortOrder::Ascending => " ↑",
                SortOrder::Descending => " ↓",
            }
        } else {
            ""
        };
        Cell::from(format!("{label}{indicator}"))
    }
}
