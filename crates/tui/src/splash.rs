use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

const LOGO: &[u8] = include_bytes!("../media/logo.png");

pub(crate) struct Splash {
    image: DynamicImage,
    rendered: Option<(u16, u16, Vec<Line<'static>>)>,
}

impl Splash {
    pub(crate) fn new() -> Result<Self> {
        Ok(Self {
            image: image::load_from_memory(LOGO)
                .context("failed to decode embedded splash logo")?,
            rendered: None,
        })
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let (columns, rows) = fitted_size(area, self.image.width(), self.image.height());
        if columns == 0 || rows == 0 {
            return;
        }

        if self
            .rendered
            .as_ref()
            .is_none_or(|(cached_columns, cached_rows, _)| {
                *cached_columns != columns as u16 || *cached_rows != rows as u16
            })
        {
            self.rendered = Some((
                columns as u16,
                rows as u16,
                converted_lines(&self.image, columns, rows),
            ));
        }

        let x = area.x + (area.width - columns as u16) / 2;
        let y = area.y + (area.height - rows as u16) / 2;
        let lines = self
            .rendered
            .as_ref()
            .expect("splash was just rendered")
            .2
            .clone();
        frame.render_widget(
            Paragraph::new(lines),
            Rect::new(x, y, columns as u16, rows as u16),
        );
    }
}

fn converted_lines(image: &DynamicImage, columns: u32, rows: u32) -> Vec<Line<'static>> {
    let resized = image
        .resize_exact(columns, rows * 2, FilterType::Nearest)
        .to_rgba8();
    let mut lines = Vec::with_capacity(rows as usize);

    for row in 0..rows {
        let mut line = Line::default();
        for column in 0..columns {
            let top = resized.get_pixel(column, row * 2);
            let bottom = resized.get_pixel(column, row * 2 + 1);
            let top_visible = top[3] >= 128;
            let bottom_visible = bottom[3] >= 128;

            let (glyph, foreground, background): (&str, Option<Color>, Option<Color>) =
                match (top_visible, bottom_visible) {
                    (false, false) => (" ", None, None),
                    (true, false) => (
                        "▀",
                        Some(Color::Rgb(top[0], top[1], top[2])),
                        Some(Color::Reset),
                    ),
                    (false, true) => (
                        "▄",
                        Some(Color::Rgb(bottom[0], bottom[1], bottom[2])),
                        Some(Color::Reset),
                    ),
                    (true, true) => (
                        "▀",
                        Some(Color::Rgb(top[0], top[1], top[2])),
                        Some(Color::Rgb(bottom[0], bottom[1], bottom[2])),
                    ),
                };

            let mut style = Style::default();
            if let Some(color) = foreground {
                style = style.fg(color);
            }
            if let Some(color) = background {
                style = style.bg(color);
            }
            line.spans.push(Span::styled(glyph, style));
        }
        lines.push(line);
    }

    lines
}

fn fitted_size(area: Rect, image_width: u32, image_height: u32) -> (u32, u32) {
    let image_cell_ratio = image_width as f32 / image_height as f32 * 2.0;
    let terminal_ratio = area.width as f32 / area.height.max(1) as f32;

    if terminal_ratio > image_cell_ratio {
        (
            (area.height as f32 * image_cell_ratio) as u32,
            area.height as u32,
        )
    } else {
        (
            area.width as u32,
            (area.width as f32 / image_cell_ratio) as u32,
        )
    }
}
