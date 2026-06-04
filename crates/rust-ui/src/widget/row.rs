//! Row layout widget — horizontal flex container.

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::Theme;
use crate::widget::Widget;

/// Horizontal arrangement of widgets.
///
/// ```rust,ignore
/// use rust_ui::widget::{row, text, button};
///
/// let r = row![text("Name:"), text("Alice")]
///     .spacing(12.0)
///     .padding(8.0);
/// ```
pub struct Row {
    id:       String,
    children: Vec<Box<dyn Widget>>,
    spacing:  f32,
    padding:  f32,
}

impl Row {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: uuid(),
            children,
            spacing: 0.0,
            padding: 0.0,
        }
    }

    pub fn spacing(mut self, s: f32) -> Self { self.spacing = s; self }
    pub fn padding(mut self, p: f32) -> Self { self.padding = p; self }
}

impl Widget for Row {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let inner = Rect::new(
            bounds.x + self.padding,
            bounds.y + self.padding,
            bounds.width  - self.padding * 2.0,
            bounds.height - self.padding * 2.0,
        );

        // Simple equal-width distribution (real layout uses taffy)
        let n = self.children.len() as f32;
        if n == 0.0 { return; }
        let total_spacing = self.spacing * (n - 1.0);
        let child_w = (inner.width - total_spacing) / n;

        let mut x = inner.x;
        for child in &self.children {
            let child_bounds = Rect::new(x, inner.y, child_w, inner.height);
            child.draw(renderer, child_bounds, theme);
            x += child_w + self.spacing;
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let inner = Rect::new(
            bounds.x + self.padding,
            bounds.y + self.padding,
            bounds.width  - self.padding * 2.0,
            bounds.height - self.padding * 2.0,
        );
        let n = self.children.len() as f32;
        if n == 0.0 { return EventStatus::Ignored; }
        let child_w = (inner.width - self.spacing * (n - 1.0)) / n;
        let mut x = inner.x;
        for child in &mut self.children {
            let cb = Rect::new(x, inner.y, child_w, inner.height);
            if child.handle_event(event, cb) == EventStatus::Consumed {
                return EventStatus::Consumed;
            }
            x += child_w + self.spacing;
        }
        EventStatus::Ignored
    }
}

/// Shorthand constructor — takes a `Vec<Box<dyn Widget>>`.
pub fn row(children: Vec<Box<dyn Widget>>) -> Row {
    Row::new(children)
}

/// Macro for ergonomic row construction.
#[macro_export]
macro_rules! row {
    ($($child:expr),* $(,)?) => {
        $crate::widget::row::row(vec![$( Box::new($child) as Box<dyn $crate::widget::Widget> ),*])
    };
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("row-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
