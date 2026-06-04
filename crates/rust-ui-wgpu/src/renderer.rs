//! VelloRenderer — implements rust_ui::render::Renderer by recording
//! draw calls into a vello Scene. The scene is later submitted to the
//! GPU by the window loop in window.rs.

use vello::kurbo::{self, Affine, RoundedRect, Stroke};
use vello::peniko::{self, Brush, Fill};
use vello::Scene;

use rust_ui::color::Color;
use rust_ui::render::{Point, Rect, Renderer, TextOptions};
use rust_ui::style::Corners;

use crate::text::TextEngine;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn to_vello_color(c: Color) -> peniko::Color {
    peniko::Color::from_rgba8(
        (c.r * 255.0) as u8,
        (c.g * 255.0) as u8,
        (c.b * 255.0) as u8,
        (c.a * 255.0) as u8,
    )
}

fn to_rounded_rect(r: Rect, corners: Corners) -> RoundedRect {
    // vello RoundedRect takes a uniform radius or per-corner via kurbo
    let rect = kurbo::Rect::new(
        r.x as f64,
        r.y as f64,
        (r.x + r.width)  as f64,
        (r.y + r.height) as f64,
    );
    // Use top-left radius as uniform approximation for now
    let radius = corners.top_left as f64;
    RoundedRect::from_rect(rect, radius)
}

// ── Renderer ─────────────────────────────────────────────────────────────────

pub struct VelloRenderer<'a> {
    scene:       &'a mut Scene,
    text_engine: &'a mut TextEngine,
    offset_stack: Vec<(f32, f32)>,
    clip_stack:   Vec<Rect>,
}

impl<'a> VelloRenderer<'a> {
    pub fn new(scene: &'a mut Scene, text_engine: &'a mut TextEngine) -> Self {
        Self {
            scene,
            text_engine,
            offset_stack: vec![(0.0, 0.0)],
            clip_stack:   vec![],
        }
    }

    fn current_offset(&self) -> (f32, f32) {
        self.offset_stack.last().copied().unwrap_or((0.0, 0.0))
    }

    fn offset_rect(&self, r: Rect) -> Rect {
        let (dx, dy) = self.current_offset();
        Rect::new(r.x + dx, r.y + dy, r.width, r.height)
    }

    fn offset_point(&self, p: Point) -> Point {
        let (dx, dy) = self.current_offset();
        Point::new(p.x + dx, p.y + dy)
    }
}

impl<'a> Renderer for VelloRenderer<'a> {
    fn fill_rect(&mut self, rect: Rect, color: Color, radius: Corners) {
        let rect = self.offset_rect(rect);
        let rr = to_rounded_rect(rect, radius);
        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(to_vello_color(color)),
            None,
            &rr,
        );
    }

    fn stroke_rect(&mut self, rect: Rect, color: Color, width: f32, radius: Corners) {
        let rect = self.offset_rect(rect);
        let rr = to_rounded_rect(rect, radius);
        let stroke = Stroke::new(width as f64);
        self.scene.stroke(
            &stroke,
            Affine::IDENTITY,
            &Brush::Solid(to_vello_color(color)),
            None,
            &rr,
        );
    }

    fn fill_circle(&mut self, center: Point, radius: f32, color: Color) {
        let center = self.offset_point(center);
        let circle = kurbo::Circle::new(
            (center.x as f64, center.y as f64),
            radius as f64,
        );
        self.scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &Brush::Solid(to_vello_color(color)),
            None,
            &circle,
        );
    }

    fn draw_line(&mut self, from: Point, to: Point, color: Color, width: f32) {
        let from = self.offset_point(from);
        let to   = self.offset_point(to);
        let line = kurbo::Line::new(
            (from.x as f64, from.y as f64),
            (to.x   as f64, to.y   as f64),
        );
        let stroke = Stroke::new(width as f64);
        self.scene.stroke(
            &stroke,
            Affine::IDENTITY,
            &Brush::Solid(to_vello_color(color)),
            None,
            &line,
        );
    }

    fn draw_text(&mut self, text: &str, pos: Point, opts: &TextOptions) -> (f32, f32) {
        let pos = self.offset_point(pos);
        let (pixels, w, h, _ascent) = self.text_engine.rasterize(text, opts.font_size, opts.color, opts.max_width);

        // Draw each glyph pixel as a tiny filled rectangle (1×1)
        // This is the simplest correct approach; a production impl would
        // use vello's glyph rendering pipeline for GPU-accelerated text.
        for px in &pixels {
            let glyph_color = Color {
                r: px.r as f32 / 255.0,
                g: px.g as f32 / 255.0,
                b: px.b as f32 / 255.0,
                a: px.a as f32 / 255.0,
            };
            let rect = Rect::new(pos.x + px.x, pos.y + px.y, 1.0, 1.0);
            let rr = to_rounded_rect(rect, Corners::ZERO);
            self.scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                &Brush::Solid(to_vello_color(glyph_color)),
                None,
                &rr,
            );
        }
        (w, h)
    }

    fn measure_text(&self, text: &str, opts: &TextOptions) -> (f32, f32, f32) {
        // We need &mut self for cosmic-text but trait says &self.
        // For now return an approximation; in production, cache results.
        let char_w = opts.font_size * 0.6;
        let w = text.chars().count() as f32 * char_w;
        let h = opts.font_size;
        // Approximate ascent as 80% of font size (typical for many fonts)
        let ascent = opts.font_size * 0.8;
        (w.min(opts.max_width.unwrap_or(f32::MAX)), h, ascent)
    }

    fn push_clip(&mut self, rect: Rect) {
        let rect = self.offset_rect(rect);
        self.clip_stack.push(rect);
        // vello clip via push_layer with a rect shape
        let clip_shape = kurbo::Rect::new(
            rect.x as f64,
            rect.y as f64,
            (rect.x + rect.width)  as f64,
            (rect.y + rect.height) as f64,
        );
        self.scene.push_layer(peniko::Mix::Clip, 1.0, Affine::IDENTITY, &clip_shape);
    }

    fn pop_clip(&mut self) {
        if self.clip_stack.pop().is_some() {
            self.scene.pop_layer();
        }
    }

    fn push_offset(&mut self, dx: f32, dy: f32) {
        let (ox, oy) = self.current_offset();
        self.offset_stack.push((ox + dx, oy + dy));
    }

    fn pop_offset(&mut self) {
        if self.offset_stack.len() > 1 {
            self.offset_stack.pop();
        }
    }

    fn begin_frame(&mut self, _size: (f32, f32)) {
        // Scene is already fresh — cleared by window.rs before calling draw()
    }

    fn end_frame(&mut self) {
        // GPU submit happens in window.rs after draw() returns
    }
}
