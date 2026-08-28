use crate::app::page::{SortOrder, UserSort, UsersPage};
use crate::ui::theme::Palette;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};

impl UsersPage {
    pub(crate) fn draw(&mut self, frame: &mut Frame, area: Rect, palette: Palette) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).split(area);
        let search = Paragraph::new(self.search.as_str())
            .style(if self.search_active {
                palette.text()
            } else {
                palette.muted()
            })
            .block(
                Block::default()
                    .title(format!(" {} (/) ", t!("users.search")))
                    .borders(Borders::ALL),
            );
        frame.render_widget(search, chunks[0]);
        if self.loading {
            frame.render_widget(
                Paragraph::new(t!("users.loading").to_string()).style(palette.muted()),
                chunks[1],
            );
            return;
        }
        let visible = self.visible_users();
        if self.users.is_empty() {
            frame.render_widget(
                Paragraph::new(t!("users.empty").to_string()).style(palette.muted()),
                chunks[1],
            );
            return;
        }
        if visible.is_empty() {
            frame.render_widget(
                Paragraph::new(t!("users.no_match").to_string()).style(palette.muted()),
                chunks[1],
            );
            return;
        }

        let header = Row::new([
            self.header(UserSort::Name),
            self.header(UserSort::Username),
            self.header(UserSort::Program),
            self.header(UserSort::Card),
            self.header(UserSort::Role),
            self.header(UserSort::Birthdate),
            self.header(UserSort::Balance),
            self.header(UserSort::Spent),
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
        self.table_state
            .select(self.selected_visible_index(&visible));
        frame.render_stateful_widget(table, chunks[1], &mut self.table_state);
    }

    fn header(&self, field: UserSort) -> Cell<'static> {
        let indicator = if self.sort_field == field {
            match self.sort_order {
                SortOrder::Ascending => " ↑",
                SortOrder::Descending => " ↓",
            }
        } else {
            ""
        };
        Cell::from(format!("{}{indicator}", field.label()))
    }
}

impl UserSort {
    fn label(self) -> String {
        t!(match self {
            Self::Name => "users_table.name",
            Self::Username => "users_table.username",
            Self::Program => "users_table.program",
            Self::Card => "users_table.card",
            Self::Role => "users_table.role",
            Self::Birthdate => "users_table.birthdate",
            Self::Balance => "users_table.balance",
            Self::Spent => "users_table.spent",
        })
        .to_string()
    }
}
