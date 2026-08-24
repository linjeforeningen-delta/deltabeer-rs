use anyhow::{Context, Result};
use crossterm::event::{self, Event};
use image::{DynamicImage, imageops::FilterType};
use insa::Insa;
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::Rect,
    style::Color,
    text::{Line, Span},
    widgets::Paragraph,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const LOGO: &[u8] = include_bytes!("../media/logo.png");
const SPLASH_DURATION: Duration = Duration::from_millis(10_250);

static SHOWN: AtomicBool = AtomicBool::new(false);

pub(crate) fn show<B>(terminal: &mut Terminal<B>, poll_interval: Duration) -> Result<()>
where
    B: Backend,
    B::Error: Send + Sync + 'static,
{
    if SHOWN.swap(true, Ordering::Relaxed) {
        return Ok(());
    }

    let image = image::load_from_memory(LOGO).context("failed to decode embedded splash logo")?;
    let started = Instant::now();

    terminal.draw(|frame| draw(frame, &image))?;

    loop {
        let elapsed = started.elapsed();
        if elapsed >= SPLASH_DURATION {
            break;
        }

        if event::poll(SPLASH_DURATION - elapsed)? {
            match event::read()? {
                Event::Key(_) => break,

                Event::Resize(_, _) => {
                    terminal.draw(|frame| draw(frame, &image))?;
                }

                _ => {}
            }
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, image: &DynamicImage) {
    let area = frame.area();
    let (columns, rows) = fitted_size(area, image.width(), image.height());
    if columns == 0 || rows == 0 {
        return;
    }

    let resized = image.resize_exact(columns * 8, rows * 16, FilterType::Triangle);
    let insa = Insa::blocks();
    let lines = insa
        .convert(&resized)
        .map(|((_, row), symbol)| (row, symbol))
        .fold(
            Vec::<Line>::with_capacity(rows as usize),
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
        );

    let x = area.x + (area.width - columns as u16) / 2;
    let y = area.y + (area.height - rows as u16) / 2;
    frame.render_widget(
        Paragraph::new(lines),
        Rect::new(x, y, columns as u16, rows as u16),
    );
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
