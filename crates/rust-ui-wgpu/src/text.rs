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
    pub fn measure(&mut self, text: &str, font_size: f32, max_width: Option<f32>) -> (f32, f32) {
        if text.is_empty() { return (0.0, font_size); }

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
        for run in buffer.layout_runs() {
            w = w.max(run.line_w);
            h += run.line_height;
        }
        (w, h.max(font_size))
    }

    /// Rasterise text glyphs and return them as RGBA pixel data.
    /// Returns (pixels: Vec<(x,y,r,g,b,a)>, width, height).
    pub fn rasterize(
        &mut self,
        text:      &str,
        font_size: f32,
        color:     rust_ui::color::Color,
        max_width: Option<f32>,
    ) -> Vec<GlyphPixel> {
        if text.is_empty() { return vec![]; }

        let metrics = Metrics::new(font_size, font_size * 1.2);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(&mut self.font_system, max_width, None);
        let attrs = Attrs::new();
        buffer.set_text(&mut self.font_system, text, attrs, Shaping::Advanced);
        buffer.shape_until_scroll(&mut self.font_system, false);

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
                    pixels.push(GlyphPixel {
                        x: x as f32,
                        y: y as f32,
                        r: color.r(),
                        g: color.g(),
                        b: color.b(),
                        a: color.a(),
                    });
                }
            },
        );

        pixels
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
