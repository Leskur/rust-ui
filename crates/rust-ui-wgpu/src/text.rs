//! Text engine — wraps cosmic-text for font shaping and layout.

use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};

pub struct TextEngine {
    pub font_system: FontSystem,
    pub swash_cache: SwashCache,
}

impl TextEngine {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }

    /// Measure text dimensions without drawing.
    /// Returns (width, height, ascent) where ascent is the distance from
    /// baseline to the top of the text bounding box.
    pub fn measure(&mut self, text: &str, font_size: f32, max_width: Option<f32>) -> (f32, f32, f32) {
        if text.is_empty() { return (0.0, font_size, font_size * 0.8); }

        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(
            &mut self.font_system,
            max_width,
            None,
        );
        let attrs = Attrs::new();
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        let mut w = 0.0_f32;
        let mut h = 0.0_f32;
        let mut max_ascent: f32 = 0.0;
        for run in buffer.layout_runs() {
            w = w.max(run.line_w);
            h += run.line_height;
            // ascent = line_top - baseline (baseline is at y=0 in cosmic-text coordinates)
            // line_top is negative (above baseline), so ascent is -line_top
            max_ascent = max_ascent.max(-run.line_top);
        }
        (w, h.max(font_size), max_ascent)
    }

    /// Rasterise text glyphs and return them as RGBA pixel data.
    /// Returns pixels with coordinates relative to the top-left of the text bounding box.
    pub fn rasterize(
        &mut self,
        text:      &str,
        font_size: f32,
        color:     rust_ui::color::Color,
        max_width: Option<f32>,
    ) -> (Vec<GlyphPixel>, f32, f32, f32) {
        if text.is_empty() { return (vec![], 0.0, font_size, font_size * 0.8); }

        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(&mut self.font_system, max_width, None);
        let attrs = Attrs::new();
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

        // Get line metrics first (same as measure)
        let mut line_top: f32 = 0.0;
        let mut line_height: f32 = font_size * 1.2;
        let mut max_width_calculated: f32 = 0.0;
        for run in buffer.layout_runs() {
            line_top = line_top.min(run.line_top); // line_top is negative
            line_height = run.line_height;
            max_width_calculated = max_width_calculated.max(run.line_w);
        }
        let ascent = -line_top; // distance from baseline to line top

        let cr = (color.r * 255.0) as u8;
        let cg = (color.g * 255.0) as u8;
        let cb = (color.b * 255.0) as u8;

        let mut pixels = Vec::new();

        buffer.draw(
            &mut self.font_system,
            &mut self.swash_cache,
            cosmic_text::Color::rgb(cr, cg, cb),
            |x, y, _w, _h, color| {
                if color.a() > 0 {
                    // Adjust y so that line_top becomes 0 (top of text block)
                    let adjusted_y = (y as f32) - line_top;
                    pixels.push(GlyphPixel {
                        x: x as f32,
                        y: adjusted_y,
                        r: color.r(),
                        g: color.g(),
                        b: color.b(),
                        a: color.a(),
                    });
                }
            },
        );

        let w = pixels.iter().map(|p| p.x).fold(0.0_f32, |a, b| a.max(b));
        let h = line_height;

        (pixels, w.max(max_width_calculated), h, ascent)
    }
}

pub struct GlyphPixel {
    pub x: f32,
    pub y: f32,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
