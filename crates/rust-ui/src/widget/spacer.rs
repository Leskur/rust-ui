//! Spacer — flexible gap that fills remaining space in a Row or Column.
//!
//! ```rust,ignore
//! row![text("Title"), spacer(), button("Settings")]
//! spacer().size(24.0)  // fixed gap
//! ```

use crate::event::EventStatus;
use crate::render::{Rect, Renderer};
use crate::style::Theme;
use crate::widget::Widget;

pub struct Spacer {
    id:   String,
    /// Fixed size. `None` means "expand to fill remaining space" (requires
    /// Row/Column to perform a flex-grow pass).
    pub(crate) fixed: Option<f32>,
}

impl Spacer {
    pub fn new() -> Self {
        Self { id: uuid(), fixed: None }
    }

    /// Force a fixed pixel size (width in Row, height in Column).
    pub fn size(mut self, px: f32) -> Self {
        self.fixed = Some(px);
        self
    }
}

impl Default for Spacer {
    fn default() -> Self { Self::new() }
}

impl Widget for Spacer {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, _renderer: &mut dyn Renderer, _bounds: Rect, _theme: &Theme) {}

    fn handle_event(&mut self, _event: &crate::event::Event, _bounds: Rect) -> EventStatus {
        EventStatus::Ignored
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        let s = self.fixed.unwrap_or(0.0);
        (s, s)
    }

    fn is_spacer(&self) -> bool { self.fixed.is_none() }
}

pub fn spacer() -> Spacer { Spacer::new() }

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("spacer-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
