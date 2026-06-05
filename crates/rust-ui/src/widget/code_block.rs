//! CodeBlock widget — renders multi-line code with a dark background and monospace font.
//!
//! ```rust,ignore
//! code_block(r#"
//!     icon("search").size(24.0).color(Color::hex("#6366f1"))
//! "#)
//! ```

use crate::color::Color;
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

// ── CodeBlock widget ───────────────────────────────────────────────────────────

pub struct CodeBlock {
    id: String,
    code: String,
    font_size: f32,
    line_height: f32,
    padding: f32,
    line_numbers: bool,
}

impl CodeBlock {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            id: uuid(),
            code: code.into(),
            font_size: 12.5,
            line_height: 1.6,
            padding: 16.0,
            line_numbers: true,
        }
    }

    pub fn font_size(mut self, s: f32) -> Self {
        self.font_size = s;
        self
    }
    pub fn line_numbers(mut self, v: bool) -> Self {
        self.line_numbers = v;
        self
    }
    pub fn no_line_numbers(mut self) -> Self {
        self.line_numbers = false;
        self
    }

    fn stripped_lines(&self) -> Vec<String> {
        let code = self.code.trim_matches('\n');
        let min_indent = code
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);
        code.lines()
            .map(|l| {
                if l.len() >= min_indent {
                    l[min_indent..].to_string()
                } else {
                    l.to_string()
                }
            })
            .collect()
    }

    fn line_px(&self) -> f32 {
        self.font_size * self.line_height
    }

    fn text_opts(&self, color: Color) -> TextOptions {
        TextOptions {
            font_size: self.font_size,
            color,
            font_family: Some("monospace".to_string()),
            bold: false,
            max_width: None,
        }
    }
}

impl Widget for CodeBlock {
    fn id(&self) -> &str {
        &self.id
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        let lines = self.stripped_lines();
        let height = self.padding * 2.0 + lines.len() as f32 * self.line_px();
        // Width: just return a default; actual width comes from layout bounds
        (480.0, height)
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, _theme: &Theme) {
        let bg = Color::hex("#13131a");
        let fg = Color::hex("#cdd6f4");
        let gutter_fg = Color::hex("#45475a");
        let radius = Corners::all(8.0);

        renderer.fill_rect(bounds, bg, radius);

        let lines = self.stripped_lines();
        let line_num_width = if self.line_numbers { 36.0 } else { 0.0 };
        let text_x = bounds.x + self.padding + line_num_width;
        let opts = self.text_opts(fg);
        let gutter_opts = self.text_opts(gutter_fg);

        for (i, line) in lines.iter().enumerate() {
            let y = bounds.y + self.padding + i as f32 * self.line_px();

            if self.line_numbers {
                let num = format!("{}", i + 1);
                let (nw, _, _) = renderer.measure_text(&num, &gutter_opts);
                let nx = bounds.x + self.padding + line_num_width - nw - 8.0;
                renderer.draw_text(&num, Point::new(nx, y), &gutter_opts);
            }

            if !line.is_empty() {
                renderer.draw_text(line, Point::new(text_x, y), &opts);
            }
        }
    }
}

/// Shorthand constructor.
pub fn code_block(code: impl Into<String>) -> CodeBlock {
    CodeBlock::new(code)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("code-block-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
