mod client;

use anyhow::Result;
use chrono::NaiveDate;
use client::{ApiClient, CreateUser, PatchUser, RoleBody, User};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Tabs, Wrap},
};
use std::{io, time::Duration};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
enum Screen {
    Dashboard,
    Users,
    Admins,
    Help,
}
#[derive(Clone, Copy, PartialEq)]
enum Section {
    Menu,
    General,
    Admin,
}
#[derive(Clone, Copy)]
enum Action {
    Resolve,
    User,
    Spend,
    Topup,
    Create,
    Update,
    Role,
}
struct App {
    api: ApiClient,
    section: Section,
    menu_selected: usize,
    screen: Screen,
    users: Vec<User>,
    admins: Vec<User>,
    stats: Option<client::Stats>,
    summary: Option<client::Summary>,
    selected: usize,
    input: String,
    prompt: Option<(Action, String)>,
    login: bool,
    login_id: String,
    login_password: String,
    login_field: usize,
    status: String,
    busy: bool,
    general_card: String,
    general_user: Option<User>,
    general_amount: String,
    general_field: usize,
}

impl App {
    fn new(base: String) -> Self {
        Self {
            api: ApiClient::new(base),
            section: Section::Menu,
            menu_selected: 0,
            screen: Screen::Dashboard,
            users: vec![],
            admins: vec![],
            stats: None,
            summary: None,
            selected: 0,
            input: String::new(),
            prompt: None,
            login: false,
            login_id: String::new(),
            login_password: String::new(),
            login_field: 0,
            status: "Connect to DeltaBeer admin API".into(),
            busy: false,
            general_card: String::new(),
            general_user: None,
            general_amount: String::new(),
            general_field: 0,
        }
    }
    async fn refresh(&mut self) {
        self.busy = true;
        let result = async {
            self.users = self.api.users().await?;
            self.admins = self.api.admins().await?;
            self.stats = self.api.stats().await.ok();
            self.summary = self.api.summary().await.ok();
            anyhow::Ok(())
        }
        .await;
        self.busy = false;
        self.status = result
            .map(|_| format!("Refreshed {} users", self.users.len()))
            .unwrap_or_else(|e| e.to_string());
    }
    fn current_user(&self) -> Option<&User> {
        self.users.get(self.selected)
    }
    fn begin(&mut self, action: Action, hint: &str) {
        self.input.clear();
        self.prompt = Some((action, hint.into()));
    }
    async fn submit(&mut self) {
        let Some((action, _)) = self.prompt.take() else {
            return;
        };
        let args: Vec<&str> = self.input.split('|').map(str::trim).collect();
        let result = match action {
            Action::Resolve => self
                .api
                .resolve(args.first().copied().unwrap_or(""))
                .await
                .map(|id| format!("Resolved: {id}")),
            Action::User => self
                .api
                .user(args.first().copied().unwrap_or(""))
                .await
                .map(|u| format!("{} · {} · balance {}", u.name, u.username, u.balance.0)),
            Action::Spend => match amount_args(&args) {
                Ok((id, n)) => self
                    .api
                    .spend(id, n)
                    .await
                    .map(|t| format!("Spend accepted: {}", t.amount.0)),
                Err(e) => Err(e),
            },
            Action::Topup => match amount_args(&args) {
                Ok((id, n)) => self
                    .api
                    .topup(id, n)
                    .await
                    .map(|t| format!("Top-up accepted: {}", t.amount.0)),
                Err(e) => Err(e),
            },
            Action::Create => match create_args(&args) {
                Ok(body) => self
                    .api
                    .create_user(body)
                    .await
                    .map(|u| format!("Created {}", u.username)),
                Err(e) => Err(e),
            },
            Action::Update => match update_args(&args) {
                Ok((id, body)) => self
                    .api
                    .update_user(id, body)
                    .await
                    .map(|u| format!("Updated {}", u.username)),
                Err(e) => Err(e),
            },
            Action::Role => {
                let role = match args
                    .get(1)
                    .copied()
                    .unwrap_or("")
                    .to_ascii_lowercase()
                    .as_str()
                {
                    "admin" => RoleBody::Admin,
                    "user" => RoleBody::User,
                    _ => return self.status = "Use: identifier|admin or identifier|user".into(),
                };
                self.api
                    .role(args.first().copied().unwrap_or(""), role)
                    .await
                    .map(|u| format!("Role updated: {}", u.username))
            }
        };
        self.status = result.unwrap_or_else(|e: anyhow::Error| e.to_string());
        if matches!(
            action,
            Action::Create | Action::Update | Action::Role | Action::Topup
        ) {
            self.refresh().await;
        }
    }
}
fn amount_args<'a>(a: &'a [&'a str]) -> Result<(&'a str, u32), anyhow::Error> {
    Ok((
        a.first()
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Use: identifier|amount"))?,
        a.get(1)
            .ok_or_else(|| anyhow::anyhow!("Use: identifier|amount"))?
            .parse()?,
    ))
}
fn create_args(a: &[&str]) -> Result<CreateUser, anyhow::Error> {
    Ok(CreateUser {
        name: a
            .first()
            .ok_or_else(|| anyhow::anyhow!("Use: name|username|program|card|YYYY-MM-DD"))?
            .to_string(),
        username: a
            .get(1)
            .ok_or_else(|| anyhow::anyhow!("missing username"))?
            .to_string(),
        program: a
            .get(2)
            .ok_or_else(|| anyhow::anyhow!("missing program"))?
            .to_string(),
        card_number: a
            .get(3)
            .ok_or_else(|| anyhow::anyhow!("missing card"))?
            .to_string(),
        birthdate: NaiveDate::parse_from_str(
            a.get(4)
                .ok_or_else(|| anyhow::anyhow!("missing birthdate"))?,
            "%Y-%m-%d",
        )?,
    })
}
fn update_args<'a>(a: &'a [&'a str]) -> Result<(&'a str, PatchUser), anyhow::Error> {
    Ok((
        a.first().ok_or_else(|| {
            anyhow::anyhow!("Use: identifier|name|username|program|card|comments")
        })?,
        PatchUser {
            name: a.get(1).filter(|x| !x.is_empty()).map(|x| x.to_string()),
            username: a.get(2).filter(|x| !x.is_empty()).map(|x| x.to_string()),
            program: a.get(3).filter(|x| !x.is_empty()).map(|x| x.to_string()),
            card_number: a.get(4).filter(|x| !x.is_empty()).map(|x| x.to_string()),
            comments: a.get(5).map(|x| x.to_string()),
        },
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    let base = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://127.0.0.1:3000".into());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new(base);
    let result = run(&mut terminal, &mut app).await;
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}
async fn run(t: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        t.draw(|f| draw(f, app))?;
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if handle(app, key).await {
                    break;
                }
            }
        }
    }
    Ok(())
}
async fn handle(a: &mut App, k: KeyEvent) -> bool {
    if a.section == Section::Menu {
        match k.code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Up | KeyCode::Down => a.menu_selected = 1 - a.menu_selected,
            KeyCode::Char('g') => a.section = Section::General,
            KeyCode::Char('a') => {
                a.section = Section::Admin;
                a.login = a.api.token.is_none();
                a.status = "Sign in to continue to the admin console".into();
            }
            KeyCode::Enter if a.menu_selected == 0 => a.section = Section::General,
            KeyCode::Enter => {
                a.section = Section::Admin;
                a.login = a.api.token.is_none();
                a.status = "Sign in to continue to the admin console".into();
            }
            _ => {}
        }
        return false;
    }

    if a.section == Section::General {
        match k.code {
            KeyCode::Esc | KeyCode::Char('b') => {
                a.section = Section::Menu;
                a.general_card.clear();
                a.general_user = None;
                a.general_amount.clear();
            }
            KeyCode::Tab => a.general_field = (a.general_field + 1) % 2,
            KeyCode::Backspace => {
                if a.general_field == 0 {
                    a.general_card.pop();
                    a.general_user = None;
                } else {
                    a.general_amount.pop();
                }
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                if a.general_field == 0 {
                    a.general_card.push(c);
                    a.general_user = None;
                } else {
                    a.general_amount.push(c);
                }
            }
            KeyCode::Enter if a.general_field == 0 => {
                a.busy = true;
                let result = a.api.user(&a.general_card).await;
                a.busy = false;
                match result {
                    Ok(user) => {
                        a.general_user = Some(user);
                        a.general_field = 1;
                        a.status = "Card recognized · enter an amount".into();
                    }
                    Err(error) => {
                        a.general_user = None;
                        a.status = error.to_string();
                    }
                }
            }
            KeyCode::Enter => match a.general_amount.parse::<u32>() {
                Ok(amount) if !a.general_card.is_empty() && amount > 0 => {
                    a.busy = true;
                    let result = a.api.spend(&a.general_card, amount).await;
                    a.busy = false;
                    match result {
                        Ok(transaction) => {
                            a.status = format!("Payment accepted · {} spent", transaction.amount.0);
                            a.general_card.clear();
                            a.general_user = None;
                            a.general_amount.clear();
                            a.general_field = 0;
                        }
                        Err(error) => a.status = error.to_string(),
                    }
                }
                _ => a.status = "Enter a card number and a positive amount".into(),
            },
            _ => {}
        }
        return false;
    }

    if a.login {
        match k.code {
            KeyCode::Esc | KeyCode::Char('b') => {
                a.section = Section::Menu;
                a.login = false;
            }
            KeyCode::Tab => a.login_field = (a.login_field + 1) % 2,
            KeyCode::Backspace => {
                if a.login_field == 0 {
                    a.login_id.pop();
                } else {
                    a.login_password.pop();
                }
            }
            KeyCode::Char(c) if !k.modifiers.contains(KeyModifiers::CONTROL) => {
                if a.login_field == 0 {
                    a.login_id.push(c)
                } else {
                    a.login_password.push(c)
                }
            }
            KeyCode::Enter => {
                if let Ok(id) = Uuid::parse_str(&a.login_id) {
                    a.busy = true;
                    let r = a.api.login(id, a.login_password.clone()).await;
                    a.busy = false;
                    match r {
                        Ok(()) => {
                            a.login = false;
                            a.status = "Authenticated · press r to refresh".into();
                            a.refresh().await
                        }
                        Err(e) => a.status = e.to_string(),
                    }
                } else {
                    a.status = "Enter a valid admin UUID".into()
                }
            }
            _ => {}
        }
        return false;
    }
    if a.prompt.is_some() {
        match k.code {
            KeyCode::Esc => a.prompt = None,
            KeyCode::Backspace => {
                a.input.pop();
            }
            KeyCode::Enter => a.submit().await,
            KeyCode::Char(c) => a.input.push(c),
            _ => {}
        }
        return false;
    }
    if matches!(k.code, KeyCode::Char('q') | KeyCode::Esc) {
        if a.section == Section::Admin {
            let _ = a.api.logout().await;
            a.section = Section::Menu;
            a.status = "Returned to main menu".into();
            return false;
        }
        return true;
    }
    match k.code {
        KeyCode::Char('r') => a.refresh().await,
        KeyCode::Char('1') => a.screen = Screen::Dashboard,
        KeyCode::Char('2') => a.screen = Screen::Users,
        KeyCode::Char('3') => a.screen = Screen::Admins,
        KeyCode::Char('h') => a.screen = Screen::Help,
        KeyCode::Down => {
            if a.selected + 1 < a.users.len() {
                a.selected += 1
            }
        }
        KeyCode::Up => a.selected = a.selected.saturating_sub(1),
        KeyCode::Char('l') => {
            let _ = a.api.logout().await;
            a.login = true;
            a.login_id.clear();
            a.login_password.clear();
        }
        KeyCode::Char('b') => {
            let _ = a.api.logout().await;
            a.section = Section::Menu;
            a.status = "Returned to main menu".into();
        }
        KeyCode::Char('v') => {
            if let Some(u) = a.current_user().cloned() {
                a.status = format!(
                    "{} · {} · card {} · balance {} · spent {}",
                    u.name, u.username, u.card_number, u.balance.0, u.spent.0
                );
            }
        }
        KeyCode::Char('x') => a.begin(Action::Resolve, "identifier"),
        KeyCode::Char('u') => a.begin(Action::User, "identifier"),
        KeyCode::Char('s') => a.begin(Action::Spend, "identifier|amount"),
        KeyCode::Char('t') => a.begin(Action::Topup, "identifier|amount"),
        KeyCode::Char('c') => a.begin(Action::Create, "name|username|program|card|YYYY-MM-DD"),
        KeyCode::Char('e') => a.begin(
            Action::Update,
            "identifier|name|username|program|card|comments",
        ),
        KeyCode::Char('a') => a.begin(Action::Role, "identifier|admin or identifier|user"),
        _ => {}
    }
    false
}

fn draw(f: &mut ratatui::Frame, a: &App) {
    let area = f.area();
    if a.section == Section::Menu {
        draw_menu(f, area, a);
        return;
    }
    if a.section == Section::General {
        draw_general(f, area, a);
        return;
    }
    if a.login {
        draw_login(f, area, a);
        return;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(5),
                Constraint::Length(2),
            ]
            .as_ref(),
        )
        .split(area);
    let titles = ["1 Dashboard", "2 Users", "3 Admins", "h Help"];
    f.render_widget(
        Tabs::new(titles)
            .select(match a.screen {
                Screen::Dashboard => 0,
                Screen::Users => 1,
                Screen::Admins => 2,
                Screen::Help => 3,
            })
            .block(Block::default().borders(Borders::ALL).title(" DeltaBeer "))
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        chunks[0],
    );
    match a.screen {
        Screen::Dashboard => dashboard(f, chunks[1], a),
        Screen::Users => user_table(f, chunks[1], a, false),
        Screen::Admins => user_table(f, chunks[1], a, true),
        Screen::Help => help(f, chunks[1]),
    }
    f.render_widget(
        Paragraph::new(Line::from(vec![
            Span::styled(
                if a.busy { " working… " } else { " " },
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(&a.status),
        ]))
        .style(Style::default().fg(Color::Gray)),
        chunks[2],
    );
    if let Some((_, hint)) = &a.prompt {
        let popup = centered(area, 60, 20);
        f.render_widget(Clear, popup);
        f.render_widget(
            Paragraph::new(vec![
                Line::styled(hint, Style::default().fg(Color::Cyan)),
                Line::raw(""),
                Line::raw(&a.input),
                Line::raw("Enter submit · Esc cancel"),
            ])
            .block(Block::default().title(" Action ").borders(Borders::ALL))
            .wrap(Wrap { trim: false }),
            popup,
        );
    }
}
fn draw_menu(f: &mut ratatui::Frame, area: Rect, a: &App) {
    let popup = centered(area, 64, 20);
    f.render_widget(Clear, popup);
    let options = ["General use", "Admin console"];
    let items = options.iter().enumerate().map(|(index, label)| {
        let marker = if index == a.menu_selected {
            "▸ "
        } else {
            "  "
        };
        Line::styled(
            format!("{marker}{label}"),
            if index == a.menu_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            },
        )
    });
    f.render_widget(
        Paragraph::new(
            std::iter::once(Line::styled(
                "DeltaBeer",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ))
            .chain(std::iter::once(Line::raw("")))
            .chain(items)
            .chain([
                Line::raw(""),
                Line::styled(
                    "↑/↓ select · Enter open · g general · a admin · q quit",
                    Style::default().fg(Color::Gray),
                ),
            ])
            .collect::<Vec<_>>(),
        )
        .block(Block::default().borders(Borders::ALL).title(" Main menu ")),
        popup,
    );
}
fn draw_general(f: &mut ratatui::Frame, area: Rect, a: &App) {
    let popup = centered(area, 70, 24);
    f.render_widget(Clear, popup);
    let card = if a.general_field == 0 {
        format!("▸ {}", a.general_card)
    } else {
        a.general_card.clone()
    };
    let amount = if a.general_field == 1 {
        format!("▸ {}", a.general_amount)
    } else {
        a.general_amount.clone()
    };
    let mut lines = vec![
        Line::styled(
            "General use",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::raw("Scan card"),
        Line::raw(card),
    ];
    if let Some(user) = &a.general_user {
        lines.extend([
            Line::raw(""),
            Line::styled("User", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw(format!("{} (@{})", user.name, user.username)),
            Line::raw(format!("Card        {}", user.card_number)),
            Line::raw(format!("Balance     {}", user.balance.0)),
        ]);
    }
    lines.extend([
        Line::raw(""),
        Line::raw("Amount to spend"),
        Line::raw(amount),
        Line::raw(""),
        Line::styled(
            "Type card number, then press Enter · Tab switches field",
            Style::default().fg(Color::Gray),
        ),
        Line::styled(
            "Enter amount to pay · b/Esc go back",
            Style::default().fg(Color::Gray),
        ),
        Line::raw(""),
        Line::raw(if a.busy {
            "Processing payment…"
        } else {
            &a.status
        }),
    ]);
    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Spend ")),
        popup,
    );
}
fn draw_login(f: &mut ratatui::Frame, area: Rect, a: &App) {
    let p = centered(area, 60, 35);
    f.render_widget(Clear, p);
    let id = if a.login_field == 0 {
        format!("▸ {}", a.login_id)
    } else {
        a.login_id.clone()
    };
    let pw = if a.login_field == 1 {
        format!("▸ {}", "•".repeat(a.login_password.len()))
    } else {
        "•".repeat(a.login_password.len())
    };
    f.render_widget(
        Paragraph::new(vec![
            Line::styled(
                "DeltaBeer Admin Console",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Line::raw(""),
            Line::raw("Admin UUID"),
            Line::raw(id),
            Line::raw("Password"),
            Line::raw(pw),
            Line::raw(""),
            Line::styled(
                "Tab switch · Enter sign in · Esc quit",
                Style::default().fg(Color::Gray),
            ),
            Line::raw(""),
            Line::raw(&a.status),
        ])
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Authenticate "),
        )
        .wrap(Wrap { trim: false }),
        p,
    );
}
fn dashboard(f: &mut ratatui::Frame, area: Rect, a: &App) {
    let s = a.summary.as_ref();
    let lines = vec![
        Line::styled(
            "Operations overview",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Line::raw(""),
        Line::raw(format!(
            "Users             {}",
            s.map(|x| x.total_users).unwrap_or(0)
        )),
        Line::raw(format!(
            "Balance           {}",
            s.map(|x| x.total_balance).unwrap_or(0)
        )),
        Line::raw(format!(
            "Spent             {}",
            s.map(|x| x.total_spent).unwrap_or(0)
        )),
        Line::raw(format!(
            "Transactions      {}",
            s.map(|x| x.total_transactions).unwrap_or(0)
        )),
        Line::raw(""),
        Line::styled(
            "r refresh · 2 users · c create · t top-up · l logout",
            Style::default().fg(Color::Gray),
        ),
    ];
    f.render_widget(
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Dashboard ")),
        area,
    );
}
fn user_table(f: &mut ratatui::Frame, area: Rect, a: &App, admins: bool) {
    let list = if admins { &a.admins } else { &a.users };
    let rows = list
        .iter()
        .enumerate()
        .map(|(i, u)| {
            Row::new(vec![
                Cell::from(if i == a.selected { "›" } else { " " }),
                Cell::from(u.name.clone()),
                Cell::from(u.username.clone()),
                Cell::from(u.card_number.to_string()),
                Cell::from(u.role.to_string()),
                Cell::from(u.balance.0.to_string()),
            ])
        })
        .collect::<Vec<_>>();
    let table = Table::new(
        rows,
        [
            Constraint::Length(2),
            Constraint::Percentage(28),
            Constraint::Percentage(20),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(12),
        ],
    )
    .header(
        Row::new(["", "Name", "Username", "Card", "Role", "Balance"]).style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    )
    .block(Block::default().borders(Borders::ALL).title(if admins {
        " Administrators "
    } else {
        " Users "
    }))
    .row_highlight_style(Style::default().bg(Color::Rgb(28, 52, 65)));
    f.render_stateful_widget(table, area, &mut ratatui::widgets::TableState::default());
    if list.is_empty() {
        f.render_widget(Paragraph::new("No data. Press r to refresh."), area);
    }
}
fn help(f: &mut ratatui::Frame, area: Rect) {
    let items = [
        "↑/↓ select user",
        "v view selected · u fetch by identifier · x resolve identifier",
        "s spend · t admin top-up · e update user · a change role",
        "c create user · r refresh · l logout · q quit",
        "Actions use | separated fields; unfinished server endpoints are surfaced as API errors.",
    ];
    f.render_widget(
        List::new(items.into_iter().map(ListItem::new))
            .block(Block::default().borders(Borders::ALL).title(" Help ")),
        area,
    );
}
fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width.saturating_sub(2));
    let h = height.min(area.height.saturating_sub(2));
    Rect {
        x: area.x + (area.width - w) / 2,
        y: area.y + (area.height - h) / 2,
        width: w,
        height: h,
    }
}
