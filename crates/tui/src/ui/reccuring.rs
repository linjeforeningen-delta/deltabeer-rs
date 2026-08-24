use crate::api::models::user::User;
use crate::ui::theme::Palette;
use ratatui::text::{Line, Span};

pub(crate) fn user_line(user: &Option<User>, palette: Palette) -> Line<'static> {
    let label = format!("{:<6}", t!("labels.user"));
    match user {
        Some(user) => Line::from(vec![
            Span::raw(label),
            Span::styled(user.name.clone(), palette.accent()),
        ]),
        None => Line::from(vec![
            Span::raw(label),
            Span::styled(t!("labels.scan_card").to_string(), palette.muted()),
        ]),
    }
}

pub(crate) fn card_line(card: &Option<String>, palette: Palette) -> Line<'static> {
    let label = format!("{:<6}", t!("labels.card"));
    match card {
        Some(card) => Line::from(vec![
            Span::raw(label),
            Span::styled(card.clone(), palette.accent()),
        ]),
        None => Line::from(vec![
            Span::raw(label),
            Span::styled(t!("labels.scan_card").to_string(), palette.muted()),
        ]),
    }
}
