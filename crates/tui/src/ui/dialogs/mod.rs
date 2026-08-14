use crate::ui::theme::Theme;
use ratatui::Frame;

pub(crate) mod user;
pub(crate) mod admin_auth;
pub(crate) mod topup;
mod menu;

pub(crate) trait DialogView {
    fn draw(
        &self,
        frame: &mut Frame,
        theme: &Theme,
    );
}