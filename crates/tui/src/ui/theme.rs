use crate::auth::AuthState;
use ratatui::style::Color;

pub struct Theme {
    pub accent: Color,
    pub border: Color,
    pub title: Color,
}


pub fn theme(auth: &AuthState) -> Theme {
    match auth {
        AuthState::Normal => normal_theme(),
        AuthState::Admin(_) => admin_theme(),
    }
}


fn admin_theme() -> Theme {
    Theme {
        accent: Color::LightRed,
        border: Color::Red,
        title: Color::LightRed,
    }
}

fn normal_theme() -> Theme {
    Theme {
        accent: Color::Cyan,
        border: Color::DarkGray,
        title: Color::Cyan,
    }
}