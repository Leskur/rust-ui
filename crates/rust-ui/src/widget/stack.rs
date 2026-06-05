//! Stack — overlapping layout (z-axis stacking).
//!
//! All children are drawn on top of each other, sharing the same bounds.
//! Useful for overlays, badges, tooltips, and background decorations.
//!
//! ```rust,ignore
//! stack![
//!     image("bg.png"),
//!     text("Overlay text").color(Color::WHITE),
//! ]
//! .alignment(Alignment::Center)
//! ```

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::Theme;
use crate::widget::Widget;

/// How children are aligned within the Stack bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

pub struct Stack {
    id: String,
    children: Vec<Box<dyn Widget>>,
    alignment: Alignment,
}

impl Stack {
    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self {
            id: uuid(),
            children,
            alignment: Alignment::TopLeft,
        }
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.alignment = a;
        self
    }

    fn child_rect(&self, child: &dyn Widget, bounds: Rect, theme: &Theme) -> Rect {
        let (cw, ch) = child.intrinsic_size(theme);
        // (0, 0) intrinsic size means "fill the stack" — used by Dialog overlays.
        if cw == 0.0 && ch == 0.0 {
            return bounds;
        }
        let x = match self.alignment {
            Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft => bounds.x,
            Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter => {
                bounds.x + (bounds.width - cw) / 2.0
            }
            Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight => {
                bounds.x + bounds.width - cw
            }
        };
        let y = match self.alignment {
            Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight => bounds.y,
            Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight => {
                bounds.y + (bounds.height - ch) / 2.0
            }
            Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight => {
                bounds.y + bounds.height - ch
            }
        };
        Rect::new(x, y, cw, ch)
    }
}

impl Widget for Stack {
    fn id(&self) -> &str {
        &self.id
    }
    fn is_container(&self) -> bool {
        true
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        for child in &self.children {
            let cb = self.child_rect(child.as_ref(), bounds, theme);
            child.draw(renderer, cb, theme);
        }
        for child in &self.children {
            let cb = self.child_rect(child.as_ref(), bounds, theme);
            child.draw_overlay(renderer, cb, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        // Pre-compute child rects before mutably iterating
        let rects: Vec<Rect> = self
            .children
            .iter()
            .map(|c| self.child_rect(c.as_ref(), bounds, &theme))
            .collect();
        // Dispatch in reverse order (top-most child gets event first).
        // For pointer-position events, hit-test so only widgets under the pointer
        // receive the event; keyboard/text events go top-most first.
        let hit_pos = match event {
            Event::MouseMove { pos }
            | Event::MouseDown { pos, .. }
            | Event::MouseUp { pos, .. }
            | Event::MouseClick { pos, .. }
            | Event::MouseDoubleClick { pos, .. }
            | Event::Scroll { pos, .. } => Some(*pos),
            _ => None,
        };

        let mut result = EventStatus::Ignored;
        for (child, cb) in self.children.iter_mut().zip(rects.iter()).rev() {
            if let Some(p) = hit_pos {
                if !child.hit_test((p.x, p.y), *cb, &theme) {
                    continue;
                }
            }
            if child.handle_event(event, *cb) == EventStatus::Consumed {
                result = EventStatus::Consumed;
                break;
            }
        }
        result
    }

    fn layout_children<'a>(&'a self, bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        // top-most (last) child first for cursor hit-testing
        self.children
            .iter()
            .rev()
            .map(|c| {
                (
                    c.as_ref() as &dyn Widget,
                    self.child_rect(c.as_ref(), bounds, theme),
                )
            })
            .collect()
    }

    fn layout_children_mut<'a>(
        &'a mut self,
        bounds: Rect,
        theme: &Theme,
    ) -> Vec<(&'a mut dyn Widget, Rect)> {
        // Pre-compute child rects before mutably iterating (avoids borrow conflicts).
        let rects: Vec<Rect> = self
            .children
            .iter()
            .map(|c| self.child_rect(c.as_ref(), bounds, theme))
            .collect();

        // top-most (last) child first
        self.children
            .iter_mut()
            .zip(rects.into_iter())
            .rev()
            .map(|(c, r)| (c.as_mut() as &mut dyn Widget, r))
            .collect()
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        // Stack size = max of all children
        self.children.iter().fold((0.0_f32, 0.0_f32), |acc, c| {
            let (w, h) = c.intrinsic_size(theme);
            (acc.0.max(w), acc.1.max(h))
        })
    }
}

pub fn stack(children: Vec<Box<dyn Widget>>) -> Stack {
    Stack::new(children)
}

/// Macro for ergonomic stack construction.
#[macro_export]
macro_rules! stack {
    ($($child:expr),* $(,)?) => {
        $crate::widget::stack::stack(vec![$( Box::new($child) as Box<dyn $crate::widget::Widget> ),*])
    };
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("stack-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
