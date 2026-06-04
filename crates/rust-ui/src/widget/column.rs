//! Column layout widget — vertical flex container.

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::{CursorStyle, Theme};
use crate::widget::Widget;

/// Vertical arrangement of widgets.
///
/// ```rust,ignore
/// use rust_ui::widget::{column, text, button};
///
/// let col = column![
///     text("Title").size(20.0),
///     text("Subtitle").size(14.0),
///     button("Action"),
/// ]
/// .spacing(12.0)
/// .padding(16.0);
/// ```
pub struct Column {
    id:       String,
    children: Vec<Box<dyn Widget>>,
    spacing:  f32,
    padding:  f32,
}

impl Column {
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

impl Column {
    fn child_rects(&self, bounds: Rect, theme: &Theme) -> Vec<Rect> {
        let inner_x = bounds.x + self.padding;
        let inner_y = bounds.y + self.padding;
        let inner_w = bounds.width - self.padding * 2.0;
        let mut y = inner_y;
        let mut rects = Vec::with_capacity(self.children.len());
        for child in &self.children {
            let (_, h) = child.intrinsic_size(theme);
            rects.push(Rect::new(inner_x, y, inner_w, h));
            y += h + self.spacing;
        }
        rects
    }
}

impl Widget for Column {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let rects = self.child_rects(
            Rect::new(0.0, 0.0, 10000.0, 10000.0),
            theme,
        );
        let w = rects.iter().map(|r| r.width).fold(0.0_f32, f32::max);
        let h = rects.last().map(|r| r.y + r.height - self.padding).unwrap_or(0.0);
        (w + self.padding * 2.0, h + self.padding)
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let rects = self.child_rects(bounds, theme);
        for (child, cb) in self.children.iter().zip(rects.iter()) {
            child.draw(renderer, *cb, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let rects = self.child_rects(bounds, &theme);
        // Mouse events must be broadcast to ALL children so every widget can
        // update its own hover/focus state (e.g. an Input unfocuses itself when
        // clicked outside). Keyboard events stop at the first consumer.
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

/// Shorthand constructor.
pub fn column(children: Vec<Box<dyn Widget>>) -> Column {
    Column::new(children)
}

/// Macro for ergonomic column construction.
#[macro_export]
macro_rules! column {
    ($($child:expr),* $(,)?) => {
        $crate::widget::column::column(vec![$( Box::new($child) as Box<dyn $crate::widget::Widget> ),*])
    };
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("col-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
