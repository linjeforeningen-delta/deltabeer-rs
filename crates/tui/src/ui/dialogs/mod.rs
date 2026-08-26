use crate::app::App;
use crate::ui::theme::{Palette, Theme};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Clear, Paragraph},
};

pub(crate) mod admin_auth;
pub(crate) mod admin_session_login;
mod grant_admin;
mod make_user;
mod menu;
mod revoke_admin;
pub(crate) mod topup;
mod update_user;
pub(crate) mod user;

pub(crate) trait DialogView {
    fn draw(&self, frame: &mut Frame, app: &App, theme: &Theme);
}

pub(crate) fn render_dialog<'a>(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    content: Vec<Line<'a>>,
    palette: Palette,
    theme: &Theme,
) {
    frame.render_widget(Clear, area);

    let popup = Paragraph::new(content)
        .style(palette.text())
        .block(theme.dialog_block(title, palette));

    frame.render_widget(popup, area);
}

pub(crate) fn action_hint(
    theme: &Theme,
    palette: Palette,
    actions: &[(&'static str, String)],
) -> Line<'static> {
    let mut spans = Vec::with_capacity(actions.len() * 2);

    for (index, (key, hint)) in actions.iter().enumerate() {
        if index > 0 {
            spans.push(Span::raw("    "));
        }
        spans.push(Span::styled(*key, theme.key_style(palette)));
        spans.push(Span::raw(format!(" {hint}")));
    }

    Line::from(spans)
}
