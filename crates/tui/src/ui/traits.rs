use crate::ui::theme::{Palette, Theme};
use ratatui::text::Line;

pub(crate) trait Content {
    fn lines(
        &self,
        theme: &Theme,
        palette: Palette,
    ) -> Vec<Line>;
}