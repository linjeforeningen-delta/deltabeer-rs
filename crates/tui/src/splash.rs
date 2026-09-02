use anyhow::{Context, Result, bail};
use image::{DynamicImage, imageops::FilterType};
use rand::random_range;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

const LOGO: &[u8] = include_bytes!("../media/logo.png");
const MARIO: &[u8] = include_bytes!("../media/mario.png");

struct SplashVariant {
    name: &'static str,
    image: DynamicImage,
    weight: u32,
}

pub(crate) struct Splash {
    variants: Vec<SplashVariant>,
    selected_variant: usize,
    rendered: Option<(u16, u16, Vec<Line<'static>>)>,
}

impl Splash {
    pub(crate) fn new() -> Result<Self> {
        // Weights are relative: the default logo is selected 90% of the time,
        // while the current Easter egg receives the remaining 10%.
        let variants = vec![
            SplashVariant {
                name: "default",
                image: image::load_from_memory(LOGO)
                    .context("failed to decode embedded default splash logo")?,
                weight: 90,
            },
            SplashVariant {
                name: "mario",
                image: image::load_from_memory(MARIO)
                    .context("failed to decode embedded Mario splash image")?,
                weight: 10,
            },
        ];

        if variants.is_empty() {
            bail!("embedded splash configuration contains no variants");
        }
        if variants
            .iter()
            .try_fold(0_u32, |total, variant| total.checked_add(variant.weight))
            .is_none_or(|total| total == 0)
        {
            bail!("embedded splash configuration has no selectable weight");
        }

        Ok(Self {
            variants,
            selected_variant: 0,
            rendered: None,
        })
    }

    pub(crate) fn begin_idle(&mut self) {
        let total_weight = self
            .variants
            .iter()
            .map(|variant| variant.weight)
            .sum::<u32>();
        let roll = random_range(0..total_weight);
        self.selected_variant = select_variant_for_roll(&self.variants, roll);
        self.rendered = None;

        tracing::debug!(
            variant = self.variants[self.selected_variant].name,
            "selected idle splash"
        );
    }

    pub(crate) fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let image = &self.variants[self.selected_variant].image;
        let (columns, rows) = fitted_size(area, image.width(), image.height());
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
                converted_lines(image, columns, rows),
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

fn select_variant_for_roll(variants: &[SplashVariant], roll: u32) -> usize {
    let mut remaining = roll;
    for (index, variant) in variants.iter().enumerate() {
        if remaining < variant.weight {
            return index;
        }
        remaining -= variant.weight;
    }

    variants
        .iter()
        .rposition(|variant| variant.weight > 0)
        .expect("splash variants must contain selectable weight")
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

#[cfg(test)]
mod tests {
    use super::{SplashVariant, select_variant_for_roll};
    use image::DynamicImage;

    fn variants() -> Vec<SplashVariant> {
        vec![
            SplashVariant {
                name: "default",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 90,
            },
            SplashVariant {
                name: "mario",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 10,
            },
        ]
    }

    #[test]
    fn weighted_roll_selects_expected_variant() {
        let variants = variants();

        assert_eq!(select_variant_for_roll(&variants, 0), 0);
        assert_eq!(select_variant_for_roll(&variants, 89), 0);
        assert_eq!(select_variant_for_roll(&variants, 90), 1);
        assert_eq!(select_variant_for_roll(&variants, 99), 1);
    }

    #[test]
    fn zero_weight_variants_are_skipped() {
        let variants = vec![
            SplashVariant {
                name: "default",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 100,
            },
            SplashVariant {
                name: "disabled",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 0,
            },
        ];

        assert_eq!(select_variant_for_roll(&variants, 99), 0);
    }
}
