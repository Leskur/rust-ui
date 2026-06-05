//! Divider — thin separator line, horizontal or vertical.
//!
//! ```rust,ignore
//! column![
//!     text("Section A"),
//!     divider(),          // horizontal (default)
//!     text("Section B"),
//! ]
//!
//! row![
//!     button("Left"),
//!     divider().vertical(),
//!     button("Right"),
//! ]
//! ```

use crate::event::EventStatus;
use crate::render::{Rect, Renderer};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DividerAxis {
    Horizontal,
    Vertical,
}

pub struct Divider {
    id:        String,
    axis:      DividerAxis,
    thickness: f32,
    margin:    f32,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            id:        uuid(),
            axis:      DividerAxis::Horizontal,
            thickness: 1.0,
            margin:    0.0,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.axis = DividerAxis::Vertical;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.axis = DividerAxis::Horizontal;
        self
    }

    pub fn thickness(mut self, t: f32) -> Self {
        self.thickness = t;
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        self.margin = m;
        self
    }
}

impl Default for Divider {
    fn default() -> Self { Self::new() }
}

impl Widget for Divider {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let color = theme.border;
        let r = match self.axis {
            DividerAxis::Horizontal => Rect::new(
                bounds.x + self.margin,
                bounds.y + (bounds.height - self.thickness) / 2.0,
                bounds.width - self.margin * 2.0,
                self.thickness,
            ),
            DividerAxis::Vertical => Rect::new(
                bounds.x + (bounds.width - self.thickness) / 2.0,
                bounds.y + self.margin,
                self.thickness,
                bounds.height - self.margin * 2.0,
            ),
        };
        renderer.fill_rect(r, color, Corners::ZERO);
    }

    fn handle_event(&mut self, _event: &crate::event::Event, _bounds: Rect) -> EventStatus {
        EventStatus::Ignored
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        match self.axis {
            DividerAxis::Horizontal => (0.0, self.thickness + self.margin * 2.0),
            DividerAxis::Vertical   => (self.thickness + self.margin * 2.0, 0.0),
        }
    }
}

pub fn divider() -> Divider { Divider::new() }

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("divider-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
