//! Icon widget — renders lucide icons as vector paths.
//!
//! Icons are rendered as vector paths via vello, so they scale perfectly.
//!
//! ```rust,ignore
//! icon("search").size(24.0).color(Color::hex("#8b8fa8"))
//! icon("chevron-right").size(16.0)
//! ```

use crate::color::Color;
use crate::render::{Rect, Renderer};
use crate::style::Theme;
use crate::widget::Widget;

// ── Icon widget ────────────────────────────────────────────────────────────────

pub struct Icon {
    id:    String,
    name:  String,
    size:  f32,
    color: Color,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id:    uuid(),
            name:  name.into(),
            size:  24.0,
            color: Color::WHITE,
        }
    }

    pub fn size(mut self, s: f32) -> Self { self.size = s; self }
    pub fn color(mut self, c: Color) -> Self { self.color = c; self }

    /// Get lucide icon SVG path data by name (hardcoded common icons).
    fn lucide_path_data(name: &str) -> Option<String> {
        // Simplified: hardcoded common icons. TODO: integrate icondata properly.
        let d = match name {
            "search" => "M21 21 L 15 15 M 17 10 A 7 7 0 1 0 3 10 A 7 7 0 0 0 17 10",
            "chevron-right" => "M9 18 L 15 12 L 9 6",
            "chevron-left" => "M15 18 L 9 12 L 15 6",
            "chevron-down" => "M6 9 L 12 15 L 18 9",
            "chevron-up" => "M18 15 L 12 9 L 6 15",
            "plus" => "M12 5 L 12 19 M5 12 L 19 12",
            "minus" => "M5 12 L 19 12",
            "x" => "M18 6 L 6 18 M6 6 L 18 18",
            "check" => "M20 6 L 9 17 L 4 12",
            "settings" => "M12.22 2 L 11.78 2 A 2 2 0 0 0 9.78 4 L 9.78 4.18 A 2 2 0 0 1 8.78 5.91 L 8.35 6.16 A 2 2 0 0 1 6.35 6.16 L 6.2 6.08 A 2 2 0 0 0 3.47 6.81 L 3.25 7.19 A 2 2 0 0 0 3.98 9.92 L 4.13 10.02 A 2 2 0 0 1 5.13 11.74 L 5.13 12.25 A 2 2 0 0 1 4.13 13.99 L 3.98 14.08 A 2 2 0 0 0 3.25 16.81 L 3.47 17.19 A 2 2 0 0 0 6.2 17.92 L 6.35 17.84 A 2 2 0 0 1 8.35 17.84 L 8.78 18.09 A 2 2 0 0 1 9.78 19.82 L 9.78 20 A 2 2 0 0 0 11.78 22 L 12.22 22 A 2 2 0 0 0 14.22 20 L 14.22 19.82 A 2 2 0 0 1 15.22 18.09 L 15.65 17.84 A 2 2 0 0 1 17.65 17.84 L 17.8 17.92 A 2 2 0 0 0 20.53 17.19 L 20.75 16.81 A 2 2 0 0 0 20.02 14.08 L 19.87 13.99 A 2 2 0 0 1 18.87 12.25 L 18.87 11.74 A 2 2 0 0 1 19.87 10.01 L 20.02 9.92 A 2 2 0 0 0 20.75 7.19 L 20.53 6.81 A 2 2 0 0 0 17.8 6.08 L 17.65 6.16 A 2 2 0 0 1 15.65 6.16 L 15.22 5.91 A 2 2 0 0 1 14.22 4.18 L 14.22 4 A 2 2 0 0 0 12.22 2 M12 15 A 3 3 0 1 0 12 9 A 3 3 0 0 0 12 15",
            "home" => "M3 9 L 12 2 L 21 9 L 21 20 A 2 2 0 0 1 19 22 L 5 22 A 2 2 0 0 1 3 20 Z",
            "user" => "M20 21 L 20 19 A 4 4 0 0 0 16 15 L 8 15 A 4 4 0 0 0 4 19 L 4 21 M12 11 A 4 4 0 1 0 12 3 A 4 4 0 0 0 12 11",
            "menu" => "M4 6 L 20 6 M4 12 L 20 12 M4 18 L 20 18",
            "bell" => "M18 8 A 6 6 0 0 0 6 8 C 6 15 3 17 3 17 L 21 17 C 21 17 18 15 18 8 M13.73 21 A 2 2 0 0 1 10.27 21",
            "heart" => "M20.84 4.61 A 5.5 5.5 0 0 0 13.06 4.61 L 12 5.67 L 10.94 4.61 A 5.5 5.5 0 0 0 3.16 12.39 L 4.22 13.45 L 12 21.23 L 19.78 13.45 L 20.84 12.39 A 5.5 5.5 0 0 0 20.84 4.61",
            "star" => "M12 2 L 15.09 8.26 L 22 9.27 L 17 14.14 L 18.18 21.02 L 12 17.77 L 5.82 21.02 L 7 14.14 L 2 9.27 L 8.91 8.26 L 12 2",
            "download" => "M21 15 L 21 19 A 2 2 0 0 1 19 21 L 5 21 A 2 2 0 0 1 3 19 L 3 15 M7 10 L 12 15 L 17 10 M12 15 L 12 3",
            "upload" => "M21 15 L 21 19 A 2 2 0 0 1 19 21 L 5 21 A 2 2 0 0 1 3 19 L 3 15 M17 8 L 12 3 L 7 8 M12 3 L 12 15",
            "trash" => "M3 6 L 21 6 M19 6 L 19 20 A 2 2 0 0 1 17 22 L 7 22 A 2 2 0 0 1 5 20 L 5 6 M8 6 L 8 4 A 2 2 0 0 1 10 2 L 14 2 A 2 2 0 0 1 16 4 L 16 6 M10 11 L 10 17 M14 11 L 14 17",
            "edit" => "M11 4 L 4 4 A 2 2 0 0 0 2 6 L 2 20 A 2 2 0 0 0 4 22 L 18 22 A 2 2 0 0 0 20 20 L 20 13 M18.5 2.5 A 2.121 2.121 0 0 1 21.5 5.5 L 12 15 L 8 16 L 9 12 L 18.5 2.5 A 2.121 2.121 0 0 0 18.5 2.5",
            "copy" => "M8 5 L 6 5 A 2 2 0 0 0 4 7 L 4 19 A 2 2 0 0 0 6 21 L 16 21 A 2 2 0 0 0 18 19 L 18 18 M8 5 A 2 2 0 0 1 10 7 L 12 7 A 2 2 0 0 1 14 7 M8 5 A 2 2 0 0 0 10 3 L 12 3 A 2 2 0 0 1 14 5 M14 5 L 16 5 A 2 2 0 0 1 18 7 L 18 10 M20 14 L 10 14 M10 14 L 13 11 M10 14 L 13 17",
            "folder" => "M22 19 A 2 2 0 0 1 20 21 L 4 21 A 2 2 0 0 1 2 19 L 2 5 A 2 2 0 0 1 4 3 L 9 3 L 11 6 L 20 6 A 2 2 0 0 1 22 8",
            "file" => "M13 2 L 6 2 A 2 2 0 0 0 4 4 L 4 20 A 2 2 0 0 0 6 22 L 18 22 A 2 2 0 0 0 20 20 L 20 9 M13 2 L 13 9 L 20 9",
            "external-link" => "M18 13 L 18 19 A 2 2 0 0 1 16 21 L 5 21 A 2 2 0 0 1 3 19 L 3 8 A 2 2 0 0 1 5 6 L 11 6 M15 3 L 21 3 L 21 9 M10 14 L 21 3",
            _ => return None,
        };
        Some(d.to_string())
    }
}

impl Widget for Icon {
    fn id(&self) -> &str { &self.id }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        (self.size, self.size)
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, _theme: &Theme) {
        let path_data = Self::lucide_path_data(&self.name).unwrap_or_default();
        if path_data.is_empty() {
            return;
        }

        // lucide icons are 24x24, scale to requested size
        let scale = self.size / 24.0;
        renderer.draw_path(&path_data, (bounds.x, bounds.y, scale), self.color);
    }
}

/// Shorthand constructor.
pub fn icon(name: impl Into<String>) -> Icon {
    Icon::new(name)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("icon-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
