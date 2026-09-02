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
use std::{
    num::NonZeroU32,
    time::{Duration, Instant},
};

const LOGO: &[u8] = include_bytes!("../media/logo.png");
const MARIO: &[u8] = include_bytes!("../media/mario.png");

struct SplashVariant {
    name: &'static str,
    image: DynamicImage,
    weight: u32,
    sizing: SplashSizing,
    motion: SplashMotion,
}

#[derive(Clone, Copy)]
enum SplashMotion {
    Centered,
    Dvd { step_interval: Duration },
}

struct DvdState {
    // Terminal glyphs cannot move between columns; only the vertical position
    // uses half-cell units because half-blocks can expose one pixel row at a time.
    x: u16,
    y_half: u32,
    dx: i8,
    dy_half: i8,
    last_step: Instant,
}

struct RenderedSplash {
    columns: u16,
    rows: u16,
    phase_0: Vec<Line<'static>>,
    phase_1: Vec<Line<'static>>,
}

#[derive(Clone, Copy)]
pub(crate) enum SplashSizing {
    Fit,
    PixelScale(NonZeroU32),
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "supported fit-scale mode covered by sizing tests and reserved for future splash variants"
        )
    )]
    PixelScaleToFit,
}

pub(crate) struct Splash {
    variants: Vec<SplashVariant>,
    selected_variant: usize,
    rendered: Option<RenderedSplash>,
    dvd_state: Option<DvdState>,
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
                motion: SplashMotion::Centered,
            },
            SplashVariant {
                name: "mario",
                image: image::load_from_memory(MARIO)
                    .context("failed to decode embedded Mario splash image")?,
                weight: 10,
                sizing: SplashSizing::PixelScale(NonZeroU32::new(1).unwrap()),
                motion: SplashMotion::Dvd {
                    step_interval: Duration::from_millis(100),
                },
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
            dvd_state: None,
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
        self.dvd_state = None;

        tracing::debug!(
            variant = self.variants[self.selected_variant].name,
            "selected idle splash"
        );
    }

    pub(crate) fn next_variant(&mut self) {
        self.selected_variant = (self.selected_variant + 1) % self.variants.len();
        self.rendered = None;
        self.dvd_state = None;

        tracing::debug!(
            variant = self.variants[self.selected_variant].name,
            "changed idle splash manually"
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
            .is_none_or(|cached| cached.columns != columns as u16 || cached.rows != rows as u16)
        {
            self.rendered = Some(RenderedSplash {
                columns: columns as u16,
                rows: rows as u16,
                phase_0: render_lines(area, image, variant.sizing, columns, rows, 0),
                phase_1: render_lines(area, image, variant.sizing, columns, rows, 1),
            });
        }

        let phase_1_rows = self
            .rendered
            .as_ref()
            .expect("splash was just rendered")
            .phase_1
            .len() as u16;
        let (x, y_half) = self.position(
            area,
            columns as u16,
            rows as u16,
            phase_1_rows,
            variant.motion,
        );
        let (whole_y, phase) = split_y_half(y_half);
        let (y, lines, rendered_rows) = {
            let rendered = self.rendered.as_ref().expect("splash was just rendered");
            let lines = if phase == 0 {
                &rendered.phase_0
            } else {
                &rendered.phase_1
            };
            (whole_y, lines.clone(), lines.len() as u16)
        };
        let y = area.y.saturating_add(y);
        frame.render_widget(
            Paragraph::new(lines),
            Rect::new(x, y, columns as u16, rendered_rows),
        );
    }

    fn position(
        &mut self,
        area: Rect,
        columns: u16,
        rows: u16,
        phase_1_rows: u16,
        motion: SplashMotion,
    ) -> (u16, u32) {
        let max_x = area.width.saturating_sub(columns);
        let max_y_cells = area.height.saturating_sub(rows);
        let max_y_half = max_y_half(area.height, rows, phase_1_rows);

        let (offset_x, offset_y) = match motion {
            SplashMotion::Centered => (u32::from(max_x / 2), u32::from(max_y_cells / 2) * 2),
            SplashMotion::Dvd { step_interval } => {
                let state = self.dvd_state.get_or_insert_with(|| DvdState {
                    x: max_x / 2,
                    y_half: max_y_half / 2,
                    dx: 1,
                    dy_half: 1,
                    last_step: Instant::now(),
                });
                clamp_dvd_position(state, max_x, max_y_half);
                if state.last_step.elapsed() >= step_interval {
                    advance_dvd_position(state, max_x, max_y_half);
                    state.last_step = Instant::now();
                }
                (u32::from(state.x), state.y_half)
            }
        };

        (area.x.saturating_add(offset_x as u16), offset_y)
    }
}

fn split_y_half(y_half: u32) -> (u16, u8) {
    ((y_half / 2) as u16, (y_half % 2) as u8)
}

fn max_y_half(area_height: u16, rows: u16, phase_1_rows: u16) -> u32 {
    let max_y_cells = area_height.saturating_sub(rows);
    if max_y_cells == 0 {
        return 0;
    }

    let max_y_cells = u32::from(max_y_cells);
    if phase_1_rows > rows {
        max_y_cells * 2
    } else {
        max_y_cells * 2
    }
}

fn advance_dvd_position(state: &mut DvdState, max_x: u16, max_y_half: u32) {
    if max_x == 0 {
        state.x = 0;
    } else {
        if (state.dx > 0 && state.x >= max_x) || (state.dx < 0 && state.x == 0) {
            state.dx = -state.dx;
        }
        state.x = if state.dx > 0 {
            state.x.saturating_add(1).min(max_x)
        } else {
            state.x.saturating_sub(1)
        };
    }

    if max_y_half == 0 {
        state.y_half = 0;
    } else {
        if (state.dy_half > 0 && state.y_half >= max_y_half)
            || (state.dy_half < 0 && state.y_half == 0)
        {
            state.dy_half = -state.dy_half;
        }
        state.y_half = if state.dy_half > 0 {
            state.y_half.saturating_add(1).min(max_y_half)
        } else {
            state.y_half.saturating_sub(1)
        };
    }
}

fn clamp_dvd_position(state: &mut DvdState, max_x: u16, max_y_half: u32) {
    state.x = state.x.min(max_x);
    state.y_half = state.y_half.min(max_y_half);
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
    vertical_phase: u8,
) -> Vec<Line<'static>> {
    match pixel_scale_for(area, image.width(), image.height(), sizing) {
        Some(scale) => {
            let pixel_width = image.width() * scale;
            let pixel_height = image.height() * scale;
            let pixels = image
                .resize_exact(pixel_width, pixel_height, FilterType::Nearest)
                .to_rgba8();
            converted_lines_from_pixels(&pixels, vertical_phase)
        }
        None => converted_lines(image, columns, rows, vertical_phase),
    }
}

fn converted_lines(
    image: &DynamicImage,
    columns: u32,
    rows: u32,
    vertical_phase: u8,
) -> Vec<Line<'static>> {
    let resized = image
        .resize_exact(columns, rows * 2, FilterType::Nearest)
        .to_rgba8();
    converted_lines_from_pixels(&resized, vertical_phase)
}

fn converted_lines_from_pixels(image: &RgbaImage, vertical_phase: u8) -> Vec<Line<'static>> {
    assert!(vertical_phase <= 1, "vertical phase must be zero or one");
    let columns = image.width();
    let rows = (image.height() + u32::from(vertical_phase)).div_ceil(2);
    let mut lines = Vec::with_capacity(rows as usize);

    for row in 0..rows {
        let mut line = Line::default();
        for column in 0..columns {
            let transparent = Rgba([0, 0, 0, 0]);
            let top = if vertical_phase == 0 {
                image
                    .get_pixel_checked(column, row * 2)
                    .unwrap_or(&transparent)
            } else {
                row.checked_mul(2)
                    .and_then(|row| row.checked_sub(1))
                    .and_then(|row| image.get_pixel_checked(column, row))
                    .unwrap_or(&transparent)
            };
            let bottom = image
                .get_pixel_checked(column, row * 2 + u32::from(1 - vertical_phase))
                .unwrap_or(&transparent);
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
        DvdState, NonZeroU32, RenderedSplash, Splash, SplashMotion, SplashSizing, SplashVariant,
        advance_dvd_position, clamp_dvd_position, converted_lines_from_pixels, render_size,
        select_variant_for_roll, split_y_half,
    };
    use image::{DynamicImage, Rgba, RgbaImage};
    use ratatui::{layout::Rect, style::Color};
    use std::time::{Duration, Instant};

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
                motion: SplashMotion::Centered,
            },
            SplashVariant {
                name: "mario",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 10,
                sizing: SplashSizing::Fit,
                motion: SplashMotion::Centered,
            },
        ]
    }

    fn splash_with_variants(variants: Vec<SplashVariant>) -> Splash {
        Splash {
            variants,
            selected_variant: 0,
            rendered: None,
            dvd_state: None,
        }
    }

    #[test]
    fn next_variant_cycles_forward_and_wraps() {
        let mut splash = splash_with_variants(variants());

        splash.next_variant();
        assert_eq!(splash.selected_variant, 1);

        splash.next_variant();
        assert_eq!(splash.selected_variant, 0);
    }

    #[test]
    fn next_variant_ignores_weights() {
        let mut splash = splash_with_variants(variants());

        splash.next_variant();

        assert_eq!(splash.selected_variant, 1);
    }

    #[test]
    fn next_variant_clears_render_cache() {
        let mut splash = splash_with_variants(variants());
        splash.rendered = Some(RenderedSplash {
            columns: 1,
            rows: 1,
            phase_0: Vec::new(),
            phase_1: Vec::new(),
        });

        splash.next_variant();

        assert!(splash.rendered.is_none());
    }

    fn dvd_state(x: u16, y_half: u32, dx: i8, dy_half: i8) -> DvdState {
        DvdState {
            x,
            y_half,
            dx,
            dy_half,
            last_step: Instant::now(),
        }
    }

    #[test]
    fn dvd_motion_advances_diagonally() {
        let mut state = dvd_state(5, 5, 1, 1);

        advance_dvd_position(&mut state, 10, 10);

        assert_eq!((state.x, state.y_half), (6, 6));
    }

    #[test]
    fn dvd_motion_bounces_at_all_edges() {
        let mut right = dvd_state(10, 5, 1, 1);
        advance_dvd_position(&mut right, 10, 10);
        assert_eq!((right.x, right.dx), (9, -1));

        let mut left = dvd_state(0, 5, -1, 1);
        advance_dvd_position(&mut left, 10, 10);
        assert_eq!((left.x, left.dx), (1, 1));

        let mut bottom = dvd_state(5, 10, 1, 1);
        advance_dvd_position(&mut bottom, 10, 10);
        assert_eq!((bottom.y_half, bottom.dy_half), (9, -1));

        let mut top = dvd_state(5, 0, 1, -1);
        advance_dvd_position(&mut top, 10, 10);
        assert_eq!((top.y_half, top.dy_half), (1, 1));
    }

    #[test]
    fn dvd_motion_freezes_axes_without_room() {
        let mut horizontal = dvd_state(5, 5, 1, 1);
        advance_dvd_position(&mut horizontal, 0, 10);
        assert_eq!((horizontal.x, horizontal.y_half), (0, 6));

        let mut vertical = dvd_state(5, 5, 1, 1);
        advance_dvd_position(&mut vertical, 10, 0);
        assert_eq!((vertical.x, vertical.y_half), (6, 0));

        let mut static_state = dvd_state(5, 5, -1, -1);
        advance_dvd_position(&mut static_state, 0, 0);
        assert_eq!((static_state.x, static_state.y_half), (0, 0));
    }

    #[test]
    fn dvd_position_is_clamped_after_resize() {
        let mut state = dvd_state(20, 18, 1, 1);

        clamp_dvd_position(&mut state, 12, 9);

        assert_eq!((state.x, state.y_half), (12, 9));
    }

    #[test]
    fn dvd_motion_does_not_clear_render_cache() {
        let mut splash = splash_with_variants(variants());
        splash.rendered = Some(RenderedSplash {
            columns: 1,
            rows: 1,
            phase_0: Vec::new(),
            phase_1: Vec::new(),
        });
        splash.dvd_state = Some(DvdState {
            last_step: Instant::now() - Duration::from_secs(1),
            ..dvd_state(0, 0, 1, 1)
        });

        let before = splash.rendered.as_ref().map(|rendered| rendered.columns);
        let position = splash.position(
            Rect::new(0, 0, 10, 10),
            1,
            1,
            1,
            SplashMotion::Dvd {
                step_interval: Duration::from_millis(1),
            },
        );

        assert_eq!(position, (1, 1));
        assert_eq!(
            splash.rendered.as_ref().map(|rendered| rendered.columns),
            before
        );
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
                motion: SplashMotion::Centered,
            },
            SplashVariant {
                name: "disabled",
                image: DynamicImage::new_rgba8(1, 1),
                weight: 0,
                sizing: SplashSizing::Fit,
                motion: SplashMotion::Centered,
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
            render_size(Rect::new(0, 0, 8, 8), &image, SplashSizing::PixelScaleToFit, ),
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
        assert_eq!(converted_lines_from_pixels(&scaled, 0).len(), 2);
    }

    #[test]
    fn odd_pixel_height_gets_an_unpaired_transparent_bottom_half() {
        let mut image = RgbaImage::new(1, 5);
        for y in 0..5 {
            image.put_pixel(0, y, Rgba([y as u8, 0, 0, 255]));
        }

        let lines = converted_lines_from_pixels(&image, 0);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[2].spans[0].content, "▀");
        assert_eq!(lines[2].spans[0].style.bg, Some(Color::Reset));
    }

    #[test]
    fn half_block_converter_shifts_by_one_pixel_row() {
        let mut image = RgbaImage::new(1, 4);
        image.put_pixel(0, 0, Rgba([255, 0, 0, 255]));
        image.put_pixel(0, 1, Rgba([0, 255, 0, 255]));
        image.put_pixel(0, 2, Rgba([0, 0, 255, 255]));
        image.put_pixel(0, 3, Rgba([255, 255, 0, 255]));

        let phase_0 = converted_lines_from_pixels(&image, 0);
        assert_eq!(phase_0.len(), 2);
        assert_eq!(phase_0[0].spans[0].content, "▀");
        assert_eq!(phase_0[0].spans[0].style.fg, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(phase_0[0].spans[0].style.bg, Some(Color::Rgb(0, 255, 0)));

        let phase_1 = converted_lines_from_pixels(&image, 1);
        assert_eq!(phase_1.len(), 3);
        assert_eq!(phase_1[0].spans[0].content, "▄");
        assert_eq!(phase_1[0].spans[0].style.fg, Some(Color::Rgb(255, 0, 0)));
        assert_eq!(phase_1[0].spans[0].style.bg, Some(Color::Reset));
        assert_eq!(phase_1[1].spans[0].content, "▀");
        assert_eq!(phase_1[1].spans[0].style.fg, Some(Color::Rgb(0, 255, 0)));
        assert_eq!(phase_1[1].spans[0].style.bg, Some(Color::Rgb(0, 0, 255)));
        assert_eq!(phase_1[2].spans[0].content, "▀");
        assert_eq!(phase_1[2].spans[0].style.fg, Some(Color::Rgb(255, 255, 0)));
        assert_eq!(phase_1[2].spans[0].style.bg, Some(Color::Reset));
    }

    #[test]
    fn half_cell_position_resolves_to_row_and_phase() {
        assert_eq!(split_y_half(0), (0, 0));
        assert_eq!(split_y_half(1), (0, 1));
        assert_eq!(split_y_half(2), (1, 0));
        assert_eq!(split_y_half(3), (1, 1));
    }
}
