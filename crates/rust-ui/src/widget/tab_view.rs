//! TabView — conditional container that shows one child at a time.
//!
//! Used for sidebar navigation, tab panels, etc.
//!
//! ```rust,ignore
//! let tabs = tab_view(vec![
//!     ("Button", button_page),
//!     ("Input", input_page),
//! ]).active("Button");
//! ```

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

pub struct TabView {
    id: String,
    pages: Vec<(String, Box<dyn Widget>)>,
    active: String,
}

impl TabView {
    pub fn new(pages: Vec<(String, Box<dyn Widget>)>) -> Self {
        let first_key = pages.first().map(|(k, _)| k.clone()).unwrap_or_default();
        Self {
            id: format!("tab-view-{}", uuid()),
            pages,
            active: first_key,
        }
    }

    pub fn active(mut self, key: impl Into<String>) -> Self {
        self.active = key.into();
        self
    }

    pub fn set_active(&mut self, key: impl Into<String>) {
        self.active = key.into();
    }
}

impl Widget for TabView {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        // Background
        renderer.fill_rect(bounds, theme.bg, Corners::ZERO);

        // Draw only the active page
        if let Some((_, widget)) = self.pages.iter().find(|(k, _)| k == &self.active) {
            widget.draw(renderer, bounds, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        // Forward events to the active page only
        for (key, widget) in &mut self.pages {
            if key == &self.active {
                return widget.handle_event(event, bounds);
            }
        }
        EventStatus::Ignored
    }

    fn layout_children<'a>(&'a self, bounds: Rect, _theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        if let Some((_, widget)) = self.pages.iter().find(|(k, _)| k == &self.active) {
            vec![(widget.as_ref() as &dyn Widget, bounds)]
        } else {
            vec![]
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        // Return the max size of all pages
        let mut max_w: f32 = 0.0;
        let mut max_h: f32 = 0.0;
        for (_, widget) in &self.pages {
            let (w, h) = widget.intrinsic_size(theme);
            max_w = max_w.max(w);
            max_h = max_h.max(h);
        }
        (max_w, max_h)
    }
}

/// Shorthand constructor.
pub fn tab_view(pages: Vec<(String, Box<dyn Widget>)>) -> TabView {
    TabView::new(pages)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("tv-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
