use crate::ui::theme::Palette;
use ratatui::text::{Line, Span};

pub(crate) fn card_line(
    card: &Option<String>,
    palette: Palette,
) -> Line<'static> {
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