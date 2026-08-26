use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};
use insa::Insa;
use ratatui::{
    Frame,
    layout::Rect,
    style::Color,
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
    let resized = image.resize_exact(columns * 8, rows * 16, FilterType::Triangle);
    let insa = Insa::blocks();
    insa.convert(&resized)
        .map(|((_, row), symbol)| (row, symbol))
        .fold(
            Vec::<Line<'static>>::with_capacity(rows as usize),
            |mut lines, (row, symbol)| {
                if lines.len() <= row as usize {
                    lines.resize_with(row as usize + 1, Line::default);
                }
                lines[row as usize].spans.push(Span::styled(
                    symbol.brush.to_string(),
                    ratatui::style::Style::default()
                        .fg(rgb(symbol.front_color))
                        .bg(symbol.back_color.map(rgb).unwrap_or(Color::Reset)),
                ));
                lines
            },
        )
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

fn rgb((red, green, blue): (u8, u8, u8)) -> Color {
    Color::Rgb(red, green, blue)
}
