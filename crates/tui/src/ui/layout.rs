use ratatui::layout::{Constraint, Layout, Rect};

pub(crate) struct AppLayout {
    pub header: Rect,
    pub body: Rect,
    pub footer: Rect,
}

pub(crate) fn app_layout(area: Rect) -> AppLayout {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(3),
    ])
        .split(area);

    AppLayout {
        header: chunks[0],
        body: chunks[1],
        footer: chunks[2],
    }
}