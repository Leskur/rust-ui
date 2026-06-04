//! Container widget — a styled box wrapping a single child.

use crate::color::Color;
use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::{Corners, Edges, Style, Theme};
use crate::widget::Widget;

/// A styled wrapper around one child widget.
///
/// ```rust,ignore
/// container(content)
///     .bg(Color::hex("#1e2130"))
///     .radius(12.0)
///     .padding(16.0)
/// ```
pub struct Container {
    id:      String,
    child:   Option<Box<dyn Widget>>,
    style:   Style,
}

impl Container {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            id:    uuid(),
            child: Some(Box::new(child)),
            style: Style::new(),
        }
    }

    pub fn empty() -> Self {
        Self { id: uuid(), child: None, style: Style::new() }
    }

    pub fn style(mut self, s: Style) -> Self { self.style = s; self }

    pub fn bg(mut self, c: impl Into<Color>) -> Self {
        self.style = self.style.bg(c); self
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.style = self.style.radius(r); self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.style = self.style.padding(Edges::all(p)); self
    }

    pub fn padding_xy(mut self, x: f32, y: f32) -> Self {
        self.style = self.style.padding_xy(x, y); self
    }

    pub fn border(mut self, color: impl Into<Color>, width: f32) -> Self {
        self.style = self.style.border_color(color).border_width(width); self
    }
}

impl Widget for Container {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        // Background
        if let Some(bg) = self.style.background {
            let radius = self.style.border
                .map(|b| b.radius)
                .unwrap_or(Corners::ZERO);
            renderer.fill_rect(bounds, bg, radius);
        }

        // Border
        if let Some(b) = self.style.border {
            if b.width > 0.0 {
                renderer.stroke_rect(bounds, b.color, b.width, b.radius);
            }
        }

        // Child with padding
        let padding = self.style.padding.unwrap_or(Edges::ZERO);
        let inner = Rect::new(
            bounds.x      + padding.left,
            bounds.y      + padding.top,
            bounds.width  - padding.left - padding.right,
            bounds.height - padding.top  - padding.bottom,
        );

        if let Some(child) = &self.child {
            child.draw(renderer, inner, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let padding = self.style.padding.unwrap_or(Edges::ZERO);
        let inner = Rect::new(
            bounds.x      + padding.left,
            bounds.y      + padding.top,
            bounds.width  - padding.left - padding.right,
            bounds.height - padding.top  - padding.bottom,
        );
        if let Some(child) = &mut self.child {
            child.handle_event(event, inner)
        } else {
            EventStatus::Ignored
        }
    }
}

/// Shorthand constructor.
pub fn container(child: impl Widget + 'static) -> Container {
    Container::new(child)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("container-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
