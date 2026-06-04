//! Row layout widget — horizontal flex container.

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::{CursorStyle, Theme};
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

impl Row {
    fn child_rects(&self, bounds: Rect, theme: &Theme) -> Vec<Rect> {
        let inner_x = bounds.x + self.padding;
        let inner_y = bounds.y + self.padding;
        let mut x = inner_x;
        let mut rects = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let (w, h) = child.intrinsic_size(theme);
            rects.push(Rect::new(x, inner_y, w, h));
            x += w + self.spacing;
        }
        rects
    }
}

impl Widget for Row {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let rects = self.child_rects(bounds, theme);
        for (child, cb) in self.children.iter().zip(rects.iter()) {
            child.draw(renderer, *cb, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let rects = self.child_rects(bounds, &theme);
        let broadcast = matches!(
            event,
            Event::MouseMove { .. } | Event::MouseDown { .. } | Event::MouseUp { .. }
        );
        let mut result = EventStatus::Ignored;
        for (child, cb) in self.children.iter_mut().zip(rects.iter()) {
            let s = child.handle_event(event, *cb);
            if s == EventStatus::Consumed {
                result = EventStatus::Consumed;
                if !broadcast { return EventStatus::Consumed; }
            }
        }
        result
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        let rects = self.child_rects(bounds, &theme);
        for (child, cb) in self.children.iter().zip(rects.iter()) {
            if cb.contains(pos.0, pos.1) {
                return child.cursor_at(pos, *cb);
            }
        }
        CursorStyle::Default
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
