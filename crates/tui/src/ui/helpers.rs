use crate::ui::theme::{Palette, Theme};
use ratatui::text::{Line, Span};

pub(crate) fn field_line<'a>(
    label: &'a str,
    value: &'a str,
    active: bool,
    theme: &Theme,
    palette: Palette,
) -> Line<'a> {
    let marker = if active { "> " } else { "  " };

    Line::from(vec![
        Span::styled(
            marker,
            if active {
                theme.selected_style(palette)
            } else {
                theme.muted_style(palette)
            },
        ),
        Span::styled(format!("{label:<10}"), theme.selected_style(palette)),
        Span::styled(
            value,
            if active {
                theme.selected_style(palette)
            } else {
                theme.muted_style(palette)
            },
        ),
    ])
}
