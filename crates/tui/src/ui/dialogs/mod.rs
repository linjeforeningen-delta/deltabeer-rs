use crate::ui::theme::Theme;
use ratatui::Frame;

pub(crate) mod admin_auth;
mod grant_admin;
mod make_user;
mod menu;
mod revoke_admin;
pub(crate) mod topup;
pub(crate) mod user;

pub(crate) trait DialogView {
    fn draw(&self, frame: &mut Frame, theme: &Theme);
}
