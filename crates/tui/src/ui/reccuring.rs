use crate::api::models::user::User;
use crate::ui::theme::Palette;
use ratatui::text::{Line, Span};

pub(crate) fn user_line(user: &Option<User>, palette: Palette) -> Line<'static> {
    match user {
        Some(user) => Line::from(vec![
            Span::raw("User  "),
            Span::styled(user.name.clone(), palette.accent()),
        ]),
        None => Line::from(vec![
            Span::raw("User  "),
            Span::styled("Scan card", palette.muted()),
        ]),
    }
}

pub(crate) fn card_line(card: &Option<String>, palette: Palette) -> Line<'static> {
    match card {
        Some(card) => Line::from(vec![
            Span::raw("Card  "),
            Span::styled(card.clone(), palette.accent()),
        ]),
        None => Line::from(vec![
            Span::raw("Card  "),
            Span::styled("Scan card", palette.muted()),
        ]),
    }
}
