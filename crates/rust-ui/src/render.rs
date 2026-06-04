//! Renderer trait — the abstraction boundary between core and backend.
//!
//! The core library only calls methods on this trait. The wgpu backend
//! (in `rust-ui-wgpu`) implements it for real GPU rendering.
//! A software/test backend can also implement it for headless testing.

use crate::color::Color;
use crate::style::{Corners, Edges};

/// Axis-aligned rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x:      f32,
    pub y:      f32,
    pub width:  f32,
    pub height: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }

    pub fn contains(&self, px: f32, py: f32) -> bool {
        px >= self.x
            && px <= self.x + self.width
            && py >= self.y
            && py <= self.y + self.height
    }
}

/// 2D point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
}

/// Text layout options passed to the renderer.
#[derive(Debug, Clone)]
pub struct TextOptions {
    pub font_size:   f32,
    pub color:       Color,
    pub font_family: Option<String>,
    pub bold:        bool,
    pub max_width:   Option<f32>,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font_size:   14.0,
            color:       Color::WHITE,
            font_family: None,
            bold:        false,
            max_width:   None,
        }
    }
}

/// The renderer trait. Implemented by each rendering backend.
///
/// All coordinates are in logical pixels (device-independent).
pub trait Renderer {
    // ── Primitives ──────────────────────────────────────────────────────────

    /// Fill a rectangle with optional rounded corners.
    fn fill_rect(&mut self, rect: Rect, color: Color, radius: Corners);

    /// Draw a rectangle border (stroke only, no fill).
    fn stroke_rect(&mut self, rect: Rect, color: Color, width: f32, radius: Corners);

    /// Fill a circle.
    fn fill_circle(&mut self, center: Point, radius: f32, color: Color);

    /// Draw a straight line.
    fn draw_line(&mut self, from: Point, to: Point, color: Color, width: f32);

    // ── Text ────────────────────────────────────────────────────────────────

    /// Draw text at the given position. Returns the measured text size.
    fn draw_text(&mut self, text: &str, pos: Point, opts: &TextOptions) -> (f32, f32);

    /// Measure text without drawing it.
    fn measure_text(&self, text: &str, opts: &TextOptions) -> (f32, f32);

    // ── Clipping ────────────────────────────────────────────────────────────

    /// Push a clip region — subsequent draws are clipped to `rect`.
    fn push_clip(&mut self, rect: Rect);

    /// Pop the most recent clip region.
    fn pop_clip(&mut self);

    // ── Transforms ──────────────────────────────────────────────────────────

    /// Push a translation offset.
    fn push_offset(&mut self, dx: f32, dy: f32);

    /// Pop the most recent translation offset.
    fn pop_offset(&mut self);

    // ── Frame lifecycle ─────────────────────────────────────────────────────

    /// Called at the start of each frame. `size` is (width, height) in logical px.
    fn begin_frame(&mut self, size: (f32, f32));

    /// Called at the end of each frame; flush to screen.
    fn end_frame(&mut self);
}
