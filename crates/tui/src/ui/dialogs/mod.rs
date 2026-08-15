use crate::ui::theme::Theme;
use ratatui::Frame;

pub(crate) mod admin_auth;
mod make_user;
mod menu;
pub(crate) mod topup;
pub(crate) mod user;

pub(crate) trait DialogView {
    fn draw(&self, frame: &mut Frame, theme: &Theme);
}
