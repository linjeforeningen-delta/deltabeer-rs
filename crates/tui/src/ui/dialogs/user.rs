use crate::{
    app::UserDialogState,
    ui::{layout::centered, theme::Theme},
};
use ratatui::{
    Frame,
    layout::Alignment,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
};

pub(crate) fn draw(
    frame: &mut Frame,
    state: &UserDialogState,
    theme: &Theme,
) {
    let area = centered(frame.area(), 56, 18);

    frame.render_widget(Clear, area);

    let role = match state.user.role {
        crate::api::models::user::Role::Admin => "Admin",
        crate::api::models::user::Role::User => "User",
    };

    let content = vec![
        Line::from(vec![
            Span::styled(
                state.user.name.as_str(),
                Style::default()
                    .fg(theme.title)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("  @{}", state.user.username)),
        ]),
        Line::from(""),
        Line::from(format!(
            "Program      {}",
            state.user.program
        )),
        Line::from(format!(
            "Card         {}",
            state.user.card_number
        )),
        Line::from(format!(
            "Role         {}",
            role
        )),
        Line::from(format!(
            "Balance      {} Δ¢",
            state.user.balance.0
        )),
        Line::from(format!(
            "Spent        {} Δ¢",
            state.user.spent.0
        )),
        Line::from(""),
        Line::from("Amount"),
        Line::styled(
            format!("> {}", state.amount),
            Style::default().fg(theme.accent),
        ),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Enter",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Spend    "),
            Span::styled(
                "T",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Top up    "),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" Close"),
        ]),
    ];

    let popup = Paragraph::new(content)
        .style(Style::default().fg(theme.accent))
        .block(
            Block::default()
                .title(" User ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border)),
        );

    frame.render_widget(popup, area);
}