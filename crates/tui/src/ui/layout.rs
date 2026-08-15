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

pub(crate) fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    Rect {
        x: area.x + area.width.saturating_sub(width) / 2,
        y: area.y + area.height.saturating_sub(height) / 2,
        width,
        height,
    }
}
