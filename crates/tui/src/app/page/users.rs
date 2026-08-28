use crate::api::request::ApiRequest;
use crate::app::{Message, TextInput};
use crate::model::{User, UserId};
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UserSort {
    Name,
    Username,
    Program,
    Card,
    Role,
    Birthdate,
    Balance,
    Spent,
}

impl UserSort {
    const ALL: [Self; 8] = [
        Self::Name,
        Self::Username,
        Self::Program,
        Self::Card,
        Self::Role,
        Self::Birthdate,
        Self::Balance,
        Self::Spent,
    ];
    fn next(self) -> Self {
        let i = Self::ALL.iter().position(|f| *f == self).unwrap();
        Self::ALL[(i + 1) % Self::ALL.len()]
    }
    fn previous(self) -> Self {
        let i = Self::ALL.iter().position(|f| *f == self).unwrap();
        Self::ALL[(i + Self::ALL.len() - 1) % Self::ALL.len()]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}
impl SortOrder {
    fn toggle(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}

#[derive(Debug)]
pub(crate) struct UsersPage {
    pub(crate) users: Vec<User>,
    pub(crate) search: TextInput,
    pub(crate) search_active: bool,
    pub(crate) sort_field: UserSort,
    pub(crate) sort_order: SortOrder,
    pub(crate) selected_user_id: Option<UserId>,
    pub(crate) loading: bool,
}

impl UsersPage {
    pub(crate) fn new() -> Self {
        Self {
            users: Vec::new(),
            search: TextInput::default(),
            search_active: false,
            sort_field: UserSort::Name,
            sort_order: SortOrder::Ascending,
            selected_user_id: None,
            loading: true,
        }
    }

    pub(crate) fn set_users(&mut self, users: Vec<User>) {
        self.users = users;
        self.loading = false;
        self.reconcile_selection();
    }

    pub(crate) fn visible_users(&self) -> Vec<&User> {
        let query = self.search.as_str().to_lowercase();
        let mut users: Vec<_> = self
            .users
            .iter()
            .filter(|u| query.is_empty() || Self::matches_search(u, &query))
            .collect();
        users.sort_by(|left, right| {
            let ordering = Self::compare(left, right, self.sort_field);
            let ordering = if self.sort_order == SortOrder::Ascending {
                ordering
            } else {
                ordering.reverse()
            };
            ordering.then_with(|| left.id.0.cmp(&right.id.0))
        });
        users
    }

    fn matches_search(user: &User, query: &str) -> bool {
        [
            user.id.to_string(),
            user.name.clone(),
            user.username.clone(),
            user.program.clone(),
            user.card_number.to_string(),
            user.role.to_string(),
            user.birthdate.to_string(),
            user.comments.clone(),
            user.balance.0.to_string(),
            user.spent.0.to_string(),
        ]
        .iter()
        .any(|field| field.to_lowercase().contains(query))
    }

    fn compare(left: &User, right: &User, field: UserSort) -> Ordering {
        match field {
            UserSort::Name => left.name.to_lowercase().cmp(&right.name.to_lowercase()),
            UserSort::Username => left
                .username
                .to_lowercase()
                .cmp(&right.username.to_lowercase()),
            UserSort::Program => left
                .program
                .to_lowercase()
                .cmp(&right.program.to_lowercase()),
            UserSort::Card => left.card_number.cmp(&right.card_number),
            UserSort::Role => left.role.to_string().cmp(&right.role.to_string()),
            UserSort::Birthdate => left.birthdate.cmp(&right.birthdate),
            UserSort::Balance => left.balance.0.cmp(&right.balance.0),
            UserSort::Spent => left.spent.0.cmp(&right.spent.0),
        }
    }

    fn reconcile_selection(&mut self) {
        let visible = self.visible_users();
        if self
            .selected_user_id
            .is_some_and(|id| !visible.iter().any(|u| u.id == id))
        {
            self.selected_user_id = None;
        }
    }

    fn move_selection(&mut self, delta: i32) {
        let visible = self.visible_users();
        if visible.is_empty() {
            self.selected_user_id = None;
            return;
        }
        let current = self
            .selected_user_id
            .and_then(|id| visible.iter().position(|u| u.id == id));
        let next = match current {
            Some(i) => (i as i32 + delta).clamp(0, visible.len() as i32 - 1) as usize,
            None => usize::from(delta >= 0).min(visible.len() - 1),
        };
        self.selected_user_id = Some(visible[next].id);
    }

    pub(crate) fn handle_key(&mut self, key: KeyEvent) -> Option<Message> {
        if self.search_active {
            match key.code {
                KeyCode::Esc => {
                    self.search.clear();
                    self.search_active = false;
                    self.reconcile_selection();
                }
                KeyCode::Backspace => {
                    self.search.backspace();
                    self.reconcile_selection();
                }
                KeyCode::Char(c) => {
                    self.search.push(c);
                    self.reconcile_selection();
                }
                KeyCode::Up => self.move_selection(-1),
                KeyCode::Down => self.move_selection(1),
                _ => {}
            }
            return None;
        }
        match key.code {
            KeyCode::Char('/') => self.search_active = true,
            KeyCode::Esc => {
                self.search.clear();
                self.reconcile_selection();
            }
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::Tab => self.sort_field = self.sort_field.next(),
            KeyCode::BackTab => self.sort_field = self.sort_field.previous(),
            KeyCode::Char('s') => self.sort_order = self.sort_order.toggle(),
            KeyCode::Char('r') => {
                self.loading = true;
                return Some(Message::ApiRequest(ApiRequest::ListUsers));
            }
            KeyCode::Enter => {
                let user = self
                    .visible_users()
                    .into_iter()
                    .find(|u| Some(u.id) == self.selected_user_id)?
                    .clone();
                return Some(Message::OpenUser(user));
            }
            _ => return None,
        }
        None
    }
}
