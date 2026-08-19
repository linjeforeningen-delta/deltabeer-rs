use crate::app::App;
use crate::ui::theme::Theme;
use ratatui::Frame;

pub(crate) mod admin_auth;
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
