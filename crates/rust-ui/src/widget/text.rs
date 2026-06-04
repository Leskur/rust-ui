//! Text widget — renders a single string with style options.

use crate::color::Color;
use crate::render::{Rect, Renderer, TextOptions};
use crate::style::Theme;
use crate::widget::Widget;

/// A styled text widget.
///
/// ```rust
/// use rust_ui::widget::text;
/// use rust_ui::color::Color;
///
/// let label = text("Hello!").size(18.0).color(Color::WHITE).bold();
/// ```
pub struct Text {
    id:      String,
    content: String,
    size:    Option<f32>,
    color:   Option<Color>,
    bold:    bool,
    family:  Option<String>,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id:      uuid(),
            content: content.into(),
            size:    None,
            color:   None,
            bold:    false,
            family:  None,
        }
    }

    pub fn size(mut self, s: f32) -> Self          { self.size = Some(s); self }
    pub fn color(mut self, c: impl Into<Color>) -> Self { self.color = Some(c.into()); self }
    pub fn bold(mut self) -> Self                  { self.bold = true; self }
    pub fn font(mut self, family: impl Into<String>) -> Self {
        self.family = Some(family.into()); self
    }
}

impl Widget for Text {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let opts = TextOptions {
            font_size:   self.size.unwrap_or(theme.font_size_md),
            color:       self.color.unwrap_or(theme.fg),
            font_family: self.family.clone(),
            bold:        self.bold,
            max_width:   Some(bounds.width),
        };
        renderer.draw_text(
            &self.content,
            crate::render::Point::new(bounds.x, bounds.y),
            &opts,
        );
    }
}

/// Shorthand constructor.
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("text-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
