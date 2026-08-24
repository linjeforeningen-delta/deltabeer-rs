use crate::{app::Page, ui::theme::Palette};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::Widget,
};

/// Draws the page border and its rounded, folder-like navigation tabs.
pub(crate) struct FolderPageFrame {
    page: Page,
    palette: Palette,
}

impl FolderPageFrame {
    pub(crate) fn new(page: Page, palette: Palette) -> Self {
        Self { page, palette }
    }

    pub(crate) fn inner(area: Rect) -> Rect {
        Rect {
            x: area.x.saturating_add(2),
            y: area.y.saturating_add(4),
            width: area.width.saturating_sub(4),
            height: area.height.saturating_sub(5),
        }
    }

    fn put(buffer: &mut Buffer, area: Rect, x: u16, y: u16, symbol: &str, style: Style) {
        if x >= area.x && x < area.right() && y >= area.y && y < area.bottom() {
            buffer[(x, y)].set_symbol(symbol).set_style(style);
        }
    }

    fn text(buffer: &mut Buffer, area: Rect, x: u16, y: u16, value: &str, style: Style) {
        for (offset, character) in value.chars().enumerate() {
            Self::put(
                buffer,
                area,
                x.saturating_add(offset as u16),
                y,
                &character.to_string(),
                style,
            );
        }
    }

    fn draw_tab(&self, buffer: &mut Buffer, area: Rect, x: u16, label: &str, active: bool) {
        let width = label.chars().count() as u16 + 6;
        let right = x.saturating_add(width.saturating_sub(1));
        let y = area.y;
        let line = if active {
            self.palette.border()
        } else {
            self.palette.muted()
        };
        let label_style = if active {
            self.palette.accent().add_modifier(Modifier::BOLD)
        } else {
            self.palette.text()
        };

        Self::put(buffer, area, x, y, "╭", line);
        for column in x.saturating_add(1)..right {
            Self::put(buffer, area, column, y, "─", line);
        }
        Self::put(buffer, area, right, y, "╮", line);

        Self::put(buffer, area, x, y.saturating_add(1), "│", line);
        Self::text(
            buffer,
            area,
            x.saturating_add(2),
            y.saturating_add(1),
            label,
            label_style,
        );
        Self::put(buffer, area, right, y.saturating_add(1), "│", line);

        if active {
            Self::put(buffer, area, x, y.saturating_add(2), "│", line);
            Self::put(buffer, area, right, y.saturating_add(2), "│", line);

            // The active tab opens into the page border on the row below the
            // closed inactive pills. Keep the first tab's left edge straight.
            let page_line_y = y.saturating_add(3);
            let bottom_left = if x == area.x { "│" } else { "╯" };
            Self::put(buffer, area, x, page_line_y, bottom_left, line);
            for column in x.saturating_add(1)..right {
                Self::put(buffer, area, column, page_line_y, " ", Style::default());
            }
            Self::put(buffer, area, right, page_line_y, "╰", line);
        } else {
            Self::put(buffer, area, x, y.saturating_add(2), "╰", line);
            for column in x.saturating_add(1)..right {
                Self::put(buffer, area, column, y.saturating_add(2), "─", line);
            }
            Self::put(buffer, area, right, y.saturating_add(2), "╯", line);
        }
    }
}

impl Widget for FolderPageFrame {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let border = self.palette.border();
        let bottom = area.bottom().saturating_sub(1);
        let top = area.y.saturating_add(3);

        for x in area.x..area.right() {
            Self::put(buffer, area, x, top, "─", border);
            Self::put(buffer, area, x, bottom, "─", border);
        }
        for y in top.saturating_add(1)..bottom {
            Self::put(buffer, area, area.x, y, "│", border);
            Self::put(buffer, area, area.right().saturating_sub(1), y, "│", border);
        }
        Self::put(buffer, area, area.x, top, "╭", border);
        Self::put(
            buffer,
            area,
            area.right().saturating_sub(1),
            top,
            "╮",
            border,
        );
        Self::put(buffer, area, area.x, bottom, "╰", border);
        Self::put(
            buffer,
            area,
            area.right().saturating_sub(1),
            bottom,
            "╯",
            border,
        );

        let gap = 1;
        let mut positions = Vec::with_capacity(Page::ALL.len());
        let mut x = area.x;
        for page in Page::ALL {
            positions.push(x);
            x = x.saturating_add(page.label().chars().count() as u16 + 6 + gap);
        }

        for (page, &x) in Page::ALL.iter().zip(&positions) {
            if *page != self.page {
                self.draw_tab(buffer, area, x, page.label(), false);
            }
        }
        // Render the selected tab last so its outline is visually in front.
        for (page, &x) in Page::ALL.iter().zip(&positions) {
            if *page == self.page {
                self.draw_tab(buffer, area, x, page.label(), true);
                break;
            }
        }
    }
}
