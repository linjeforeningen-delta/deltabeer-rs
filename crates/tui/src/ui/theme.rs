use crate::auth::AuthState;
use ratatui::layout::Alignment;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, Padding};

#[derive(Clone, Copy)]
pub(crate) struct Palette {
    text: Color,
    muted: Color,
    accent: Color,
    border: Color,

    success: Color,
    warning: Color,
    error: Color,
}

impl Palette {
    pub(crate) fn text(self) -> Style {
        Style::default().fg(self.text)
    }

    pub(crate) fn muted(self) -> Style {
        Style::default().fg(self.muted)
    }

    pub(crate) fn accent(self) -> Style {
        Style::default().fg(self.accent)
    }

    pub(crate) fn border(self) -> Style {
        Style::default().fg(self.border)
    }

    pub(crate) fn success(self) -> Style {
        Style::default().fg(self.success)
    }

    pub(crate) fn warning(self) -> Style {
        Style::default().fg(self.warning)
    }

    pub(crate) fn error(self) -> Style {
        Style::default().fg(self.error)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Theme {
    normal: Palette,
    admin: Palette,
    dimmed: Palette,
}

impl Theme {
    pub(crate) fn active(&self, auth: &AuthState) -> Palette {
        match auth {
            AuthState::Normal => self.normal,
            AuthState::Admin(_) => self.admin,
        }
    }

    pub(crate) fn normal(&self) -> Palette {
        self.normal
    }

    pub(crate) fn admin(&self) -> Palette {
        self.admin
    }

    pub(crate) fn dimmed(&self) -> Palette {
        self.dimmed
    }

    pub(crate) fn dialog_block<'a>(&self, title: &'a str, palette: Palette) -> Block<'a> {
        Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .title_style(self.title_style(palette))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(palette.border())
            .padding(Padding::new(2, 2, 1, 1))
    }

    pub(crate) fn page_block<'a>(&self, title: &'a str, palette: Palette) -> Block<'a> {
        Block::default()
            .title(title)
            .title_style(self.title_style(palette))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(palette.border())
            .padding(Padding::new(2, 2, 1, 1))
    }

    pub(crate) fn title_style(&self, palette: Palette) -> Style {
        palette.accent().add_modifier(Modifier::BOLD)
    }

    pub(crate) fn muted_style(&self, palette: Palette) -> Style {
        palette.muted()
    }

    pub(crate) fn selected_style(&self, palette: Palette) -> Style {
        palette.accent().add_modifier(Modifier::BOLD)
    }

    pub(crate) fn key_style(&self, palette: Palette) -> Style {
        palette.accent().add_modifier(Modifier::BOLD)
    }
}

pub(crate) const THEME: Theme = Theme {
    normal: Palette {
        text: Color::White,
        muted: Color::DarkGray,
        accent: Color::Cyan,
        border: Color::DarkGray,
        success: Color::Green,
        warning: Color::Yellow,
        error: Color::LightRed,
    },
    admin: Palette {
        text: Color::White,
        muted: Color::DarkGray,
        accent: Color::LightRed,
        border: Color::Red,
        success: Color::Green,
        warning: Color::Yellow,
        error: Color::LightRed,
    },
    dimmed: Palette {
        text: Color::DarkGray,
        muted: Color::DarkGray,
        accent: Color::DarkGray,
        border: Color::DarkGray,
        success: Color::Green,
        warning: Color::Yellow,
        error: Color::LightRed,
    },
};
