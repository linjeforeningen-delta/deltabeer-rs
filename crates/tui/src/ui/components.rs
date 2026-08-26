use crate::model::User;
use crate::ui::theme::Palette;
use ratatui::text::{Line, Span};

fn label_width() -> usize {
    [t!("labels.user"), t!("labels.card")]
        .iter()
        .map(|label| label.chars().count())
        .max()
        .unwrap_or(0)
}

pub(crate) fn user_line(user: &Option<User>, palette: Palette) -> Line<'static> {
    let width = label_width();
    let label = format!("{:<width$} ", t!("labels.user"));
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
    let width = label_width();
    let label = format!("{:<width$} ", t!("labels.card"));
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
