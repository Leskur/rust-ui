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

impl Row {
    fn child_rects(&self, bounds: Rect, theme: &Theme) -> Vec<Rect> {
        let inner_x = bounds.x + self.padding;
        let inner_y = bounds.y + self.padding;
        let available_w = bounds.width - self.padding * 2.0;
        let available_h = bounds.height - self.padding * 2.0;

        // First pass: measure fixed children and count flex spacers
        let mut sizes: Vec<(f32, f32)> = self.children.iter()
            .map(|c| c.intrinsic_size(theme))
            .collect();
        let spacer_count = self.children.iter().filter(|c| c.is_spacer()).count();
        let spacing_total = if self.children.is_empty() { 0.0 }
            else { self.spacing * (self.children.len() - 1) as f32 };
        let fixed_w: f32 = self.children.iter().zip(sizes.iter())
            .filter(|(c, _)| !c.is_spacer())
            .map(|(_, (w, _))| w)
            .sum::<f32>() + spacing_total;
        let flex_w = if spacer_count > 0 {
            ((available_w - fixed_w) / spacer_count as f32).max(0.0)
        } else { 0.0 };

        // Apply flex width to spacers
        for (child, size) in self.children.iter().zip(sizes.iter_mut()) {
            if child.is_spacer() {
                size.0 = flex_w;
            }
        }

        // Second pass: place children
        let mut x = inner_x;
        let mut rects = Vec::with_capacity(self.children.len());
        for (i, (w, h)) in sizes.iter().enumerate() {
            let h = if *h == 0.0 { available_h } else { *h };
            rects.push(Rect::new(x, inner_y, *w, h));
            x += w + if i + 1 < sizes.len() { self.spacing } else { 0.0 };
        }
        rects
    }
}

impl Widget for Row {
    fn id(&self) -> &str { &self.id }
    fn is_container(&self) -> bool { true }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let spacing_total = if self.children.is_empty() { 0.0 }
            else { self.spacing * (self.children.len() - 1) as f32 };
        let total_w: f32 = self.children.iter()
            .map(|c| c.intrinsic_size(theme).0)
            .sum::<f32>() + spacing_total;
        let max_h: f32 = self.children.iter()
            .map(|c| c.intrinsic_size(theme).1)
            .fold(0.0_f32, f32::max);
        (total_w + self.padding * 2.0, max_h + self.padding * 2.0)
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

    fn layout_children<'a>(&'a self, bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        let rects = self.child_rects(bounds, theme);
        self.children.iter()
            .zip(rects.into_iter())
            .map(|(c, r)| (c.as_ref() as &dyn Widget, r))
            .collect()
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
