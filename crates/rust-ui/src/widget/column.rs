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
        let inner_w = bounds.width  - self.padding * 2.0;
        let available_h = bounds.height - self.padding * 2.0;

        // First pass: measure fixed children
        let mut heights: Vec<f32> = self.children.iter()
            .map(|c| c.intrinsic_size(theme).1)
            .collect();
        let spacer_count = self.children.iter().filter(|c| c.is_spacer()).count();
        let spacing_total = if self.children.is_empty() { 0.0 }
            else { self.spacing * (self.children.len() - 1) as f32 };
        let fixed_h: f32 = self.children.iter().zip(heights.iter())
            .filter(|(c, _)| !c.is_spacer())
            .map(|(_, h)| h)
            .sum::<f32>() + spacing_total;
        let flex_h = if spacer_count > 0 {
            ((available_h - fixed_h) / spacer_count as f32).max(0.0)
        } else { 0.0 };

        for (child, h) in self.children.iter().zip(heights.iter_mut()) {
            if child.is_spacer() { *h = flex_h; }
        }

        // Second pass: place children
        let mut y = inner_y;
        let mut rects = Vec::with_capacity(self.children.len());
        for (i, h) in heights.iter().enumerate() {
            rects.push(Rect::new(inner_x, y, inner_w, *h));
            y += h + if i + 1 < heights.len() { self.spacing } else { 0.0 };
        }
        rects
    }
}

impl Widget for Column {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let spacing_total = if self.children.is_empty() { 0.0 }
            else { self.spacing * (self.children.len() - 1) as f32 };
        let max_w: f32 = self.children.iter()
            .map(|c| c.intrinsic_size(theme).0)
            .fold(0.0_f32, f32::max);
        let total_h: f32 = self.children.iter()
            .map(|c| c.intrinsic_size(theme).1)
            .sum::<f32>() + spacing_total;
        (max_w + self.padding * 2.0, total_h + self.padding * 2.0)
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
