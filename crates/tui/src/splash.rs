use anyhow::{Context, Result, bail};
use image::{DynamicImage, Rgba, RgbaImage, imageops::FilterType};
use rand::random_range;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
use std::num::NonZeroU32;

const LOGO: &[u8] = include_bytes!("../media/logo.png");
const MARIO: &[u8] = include_bytes!("../media/mario.png");

struct SplashVariant {
    name: &'static str,
    image: DynamicImage,
    weight: u32,
    sizing: SplashSizing,
}

#[derive(Clone, Copy)]
pub(crate) enum SplashSizing {
    Fit,
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "supported fixed-scale mode reserved for future pixel-art splash variants"
        )
    )]
    PixelScale(NonZeroU32),
    PixelScaleToFit,
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
                sizing: SplashSizing::Fit,
            },
            SplashVariant {
                name: "mario",
                image: image::load_from_memory(MARIO)
                    .context("failed to decode embedded Mario splash image")?,
                weight: 10,
                sizing: SplashSizing::PixelScaleToFit,
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
        let variant = &self.variants[self.selected_variant];
        let image = &variant.image;
        let (columns, rows) = render_size(area, image, variant.sizing);
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
                render_lines(area, image, variant.sizing, columns, rows),
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

fn render_size(area: Rect, image: &DynamicImage, sizing: SplashSizing) -> (u32, u32) {
    if let Some(scale) = pixel_scale_for(area, image.width(), image.height(), sizing) {
        let pixel_width = image
            .width()
            .checked_mul(scale)
            .expect("scaled splash width overflowed u32");
        let pixel_height = image
            .height()
            .checked_mul(scale)
            .expect("scaled splash height overflowed u32");
        return (pixel_width, pixel_height.div_ceil(2));
    }

    fitted_size(area, image.width(), image.height())
}

fn pixel_scale_for(
    area: Rect,
    image_width: u32,
    image_height: u32,
    sizing: SplashSizing,
) -> Option<u32> {
    let largest = largest_integer_scale(area, image_width, image_height);
    match sizing {
        SplashSizing::Fit => None,
        SplashSizing::PixelScale(requested) => {
            let scale = requested.get().min(largest);
            (scale > 0).then_some(scale)
        }
        SplashSizing::PixelScaleToFit => (largest > 0).then_some(largest),
    }
}

fn largest_integer_scale(area: Rect, image_width: u32, image_height: u32) -> u32 {
    if image_width == 0 || image_height == 0 {
        return 0;
    }

    let available_pixel_height = u32::from(area.height)
        .checked_mul(2)
        .expect("terminal height should fit in rendered pixel dimensions");
    (u32::from(area.width) / image_width).min(available_pixel_height / image_height)
}

fn render_lines(
    area: Rect,
    image: &DynamicImage,
    sizing: SplashSizing,
    columns: u32,
    rows: u32,
) -> Vec<Line<'static>> {
    match pixel_scale_for(area, image.width(), image.height(), sizing) {
        Some(scale) => {
            let pixel_width = image.width() * scale;
            let pixel_height = image.height() * scale;
            let pixels = image
                .resize_exact(pixel_width, pixel_height, FilterType::Nearest)
                .to_rgba8();
            converted_lines_from_pixels(&pixels)
        }
        None => converted_lines(image, columns, rows),
    }
}

fn converted_lines(image: &DynamicImage, columns: u32, rows: u32) -> Vec<Line<'static>> {
    let resized = image
        .resize_exact(columns, rows * 2, FilterType::Nearest)
        .to_rgba8();
    converted_lines_from_pixels(&resized)
}

fn converted_lines_from_pixels(image: &RgbaImage) -> Vec<Line<'static>> {
    let columns = image.width();
    let rows = image.height().div_ceil(2);
    let mut lines = Vec::with_capacity(rows as usize);

    for row in 0..rows {
        let mut line = Line::default();
        for column in 0..columns {
            let top = image.get_pixel(column, row * 2);
            let bottom = image
                .get_pixel_checked(column, row * 2 + 1)
                .copied()
                .unwrap_or(Rgba([0, 0, 0, 0]));
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
    use super::{
        NonZeroU32, SplashSizing, SplashVariant, converted_lines_from_pixels, render_size,
        select_variant_for_roll,
    };
    use image::{DynamicImage, Rgba, RgbaImage};
    use ratatui::{layout::Rect, style::Color};

    fn scale(value: u32) -> NonZeroU32 {
        NonZeroU32::new(value).expect("test scale must be positive")
    }

    fn variants() -> Vec<SplashVariant> {
        vec![
            SplashVariant {
                name: "default",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 90,
                sizing: SplashSizing::Fit,
            },
            SplashVariant {
                name: "mario",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 10,
                sizing: SplashSizing::Fit,
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
                sizing: SplashSizing::Fit,
            },
            SplashVariant {
                name: "disabled",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 0,
                sizing: SplashSizing::Fit,
            },
        ];

        assert_eq!(select_variant_for_roll(&variants, 99), 0);
    }

    #[test]
    fn fixed_pixel_scale_uses_integer_terminal_dimensions() {
        let image = DynamicImage::new_rgba8(16, 16);

        assert_eq!(
            render_size(
                Rect::new(0, 0, 80, 24),
                &image,
                SplashSizing::PixelScale(scale(2)),
            ),
            (32, 16)
        );
    }

    #[test]
    fn pixel_scale_to_fit_uses_largest_integer_scale() {
        let image = DynamicImage::new_rgba8(16, 16);

        assert_eq!(
            render_size(
                Rect::new(0, 0, 80, 24),
                &image,
                SplashSizing::PixelScaleToFit,
            ),
            (48, 24)
        );
    }

    #[test]
    fn pixel_scale_to_fit_falls_back_when_native_size_does_not_fit() {
        let image = DynamicImage::new_rgba8(16, 16);

        assert_eq!(
            render_size(Rect::new(0, 0, 8, 8), &image, SplashSizing::PixelScaleToFit,),
            (8, 4)
        );
    }

    #[test]
    fn oversized_fixed_scale_is_reduced_to_the_largest_fitting_scale() {
        let image = DynamicImage::new_rgba8(16, 16);

        assert_eq!(
            render_size(
                Rect::new(0, 0, 80, 24),
                &image,
                SplashSizing::PixelScale(scale(9)),
            ),
            (48, 24)
        );
    }

    #[test]
    fn integer_scaling_replicates_each_source_pixel() {
        let mut image = RgbaImage::new(2, 2);
        image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        image.put_pixel(1, 0, Rgba([0, 255, 0, 255]));
        image.put_pixel(0, 1, Rgba([0, 0, 255, 255]));
        image.put_pixel(1, 1, Rgba([255, 255, 0, 255]));
        let scaled = DynamicImage::ImageRgba8(image)
            .resize_exact(4, 4, image::imageops::FilterType::Nearest)
            .to_rgba8();

        let expected = [
            [
                [255, 0, 0, 255],
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 255, 0, 255],
            ],
            [
                [255, 0, 0, 255],
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0, 255, 0, 255],
            ],
            [
                [0, 0, 255, 255],
                [0, 0, 255, 255],
                [255, 255, 0, 255],
                [255, 255, 0, 255],
            ],
            [
                [0, 0, 255, 255],
                [0, 0, 255, 255],
                [255, 255, 0, 255],
                [255, 255, 0, 255],
            ],
        ];
        for y in 0..4 {
            for x in 0..4 {
                assert_eq!(scaled.get_pixel(x, y).0, expected[y as usize][x as usize]);
            }
        }
        assert_eq!(converted_lines_from_pixels(&scaled).len(), 2);
    }

    #[test]
    fn odd_pixel_height_gets_an_unpaired_transparent_bottom_half() {
        let mut image = RgbaImage::new(1, 5);
        for y in 0..5 {
            image.put_pixel(0, y, Rgba([y as u8, 0, 0, 255]));
        }

        let lines = converted_lines_from_pixels(&image);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2].spans[0].content, "▀");
        assert_eq!(lines[2].spans[0].style.bg, Some(Color::Reset));
    }
}
