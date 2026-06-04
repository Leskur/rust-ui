//! Column layout widget — vertical flex container.

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::Theme;
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

impl Widget for Column {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let inner = Rect::new(
            bounds.x + self.padding,
            bounds.y + self.padding,
            bounds.width  - self.padding * 2.0,
            bounds.height - self.padding * 2.0,
        );

        let n = self.children.len() as f32;
        if n == 0.0 { return; }
        let total_spacing = self.spacing * (n - 1.0);
        let child_h = (inner.height - total_spacing) / n;

        let mut y = inner.y;
        for child in &self.children {
            let cb = Rect::new(inner.x, y, inner.width, child_h);
            child.draw(renderer, cb, theme);
            y += child_h + self.spacing;
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
        let child_h = (inner.height - self.spacing * (n - 1.0)) / n;
        let mut y = inner.y;
        for child in &mut self.children {
            let cb = Rect::new(inner.x, y, inner.width, child_h);
            if child.handle_event(event, cb) == EventStatus::Consumed {
                return EventStatus::Consumed;
            }
            y += child_h + self.spacing;
        }
        EventStatus::Ignored
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
