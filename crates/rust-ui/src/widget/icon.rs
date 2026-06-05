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
    /// Using simplified paths with only M, L, C, Z commands (no A arcs).
    fn lucide_path_data(name: &str) -> Option<String> {
        let d = match name {
            // Navigation - simple lines
            "search" => "M15 15 L 21 21 M 10 17 C 13.866 17 17 13.866 17 10 C 17 6.134 13.866 3 10 3 C 6.134 3 3 6.134 3 10 C 3 13.866 6.134 17 10 17 Z",
            "chevron-right" => "M9 18 L 15 12 L 9 6",
            "chevron-left" => "M15 18 L 9 12 L 15 6",
            "chevron-down" => "M6 9 L 12 15 L 18 9",
            "chevron-up" => "M18 15 L 12 9 L 6 15",

            // Actions
            "plus" => "M12 5 L 12 19 M 5 12 L 19 12",
            "minus" => "M5 12 L 19 12",
            "x" => "M18 6 L 6 18 M 6 6 L 18 18",
            "check" => "M20 6 L 9 17 L 4 12",

            // UI Elements
            "settings" => "M12 15 C 13.657 15 15 13.657 15 12 C 15 10.343 13.657 9 12 9 C 10.343 9 9 10.343 9 12 C 9 13.657 10.343 15 12 15 Z M12 1 L 12 3 M 12 21 L 12 23 M 4.22 4.22 L 5.64 5.64 M 18.36 18.36 L 19.78 19.78 M 1 12 L 3 12 M 21 12 L 23 12 M 4.22 19.78 L 5.64 18.36 M 18.36 5.64 L 19.78 4.22",
            "menu" => "M4 6 L 20 6 M 4 12 L 20 12 M 4 18 L 20 18",
            "bell" => "M18 8 C 18 5 15 3 12 3 C 9 3 6 5 6 8 C 6 14 3 16 3 16 L 21 16 C 21 16 18 14 18 8 M 13.73 21 C 13.73 21 12.1 22 12 22 C 11.9 22 10.27 21 10.27 21",
            "home" => "M3 9 L 12 2 L 21 9 L 21 20 C 21 21 20 22 19 22 L 5 22 C 4 22 3 21 3 20 Z",

            // User
            "user" => "M20 21 L 20 19 C 20 17 18 15 16 15 L 8 15 C 6 15 4 17 4 19 L 4 21 M 12 11 C 14 11 16 9 16 7 C 16 5 14 3 12 3 C 10 3 8 5 8 7 C 8 9 10 11 12 11",

            // Heart (simplified)
            "heart" => "M12 21 C 12 21 4 16 4 10 C 4 7 6 4 9 4 C 11 4 12 5 12 5 C 12 5 13 4 15 4 C 18 4 20 7 20 10 C 20 16 12 21 12 21 Z",

            // Star (simplified)
            "star" => "M12 2 L 15 9 L 22 9 L 17 14 L 19 21 L 12 17 L 5 21 L 7 14 L 2 9 L 9 9 Z",

            // Transfer
            "download" => "M21 15 L 21 19 C 21 20 20 21 19 21 L 5 21 C 4 21 3 20 3 19 L 3 15 M 7 10 L 12 15 L 17 10 M 12 15 L 12 3",
            "upload" => "M21 15 L 21 19 C 21 20 20 21 19 21 L 5 21 C 4 21 3 20 3 19 L 3 15 M 17 8 L 12 3 L 7 8 M 12 3 L 12 15",
            "external-link" => "M18 13 L 18 19 C 18 20 17 21 16 21 L 5 21 C 4 21 3 20 3 19 L 3 8 C 3 7 4 6 5 6 L 11 6 M 15 3 L 21 3 L 21 9 M 10 14 L 21 3",
            "trash" => "M3 6 L 21 6 M 19 6 L 19 20 C 19 21 18 22 17 22 L 7 22 C 6 22 5 21 5 20 L 5 6 M 8 6 L 8 4 C 8 3 9 2 10 2 L 14 2 C 15 2 16 3 16 4 L 16 6 M 10 11 L 10 17 M 14 11 L 14 17",

            // Files
            "folder" => "M22 19 C 22 20 21 21 20 21 L 4 21 C 3 21 2 20 2 19 L 2 5 C 2 4 3 3 4 3 L 9 3 L 11 6 L 20 6 C 21 6 22 7 22 8 Z",
            "file" => "M13 2 L 6 2 C 5 2 4 3 4 4 L 4 20 C 4 21 5 22 6 22 L 18 22 C 19 22 20 21 20 20 L 20 9 L 13 2 Z M 13 2 L 13 9 L 20 9",
            "edit" => "M11 4 L 4 4 C 3 4 2 5 2 6 L 2 20 C 2 21 3 22 4 22 L 18 22 C 19 22 20 21 20 20 L 20 13 M 18.5 2.5 L 12 9 L 8 10 L 9 6 L 15.5 2.5 C 16.5 1.5 17.5 1.5 18.5 2.5 Z",
            "copy" => "M20 8 L 20 18 C 20 19 19 20 18 20 L 8 20 C 7 20 6 19 6 18 L 6 8 C 6 7 7 6 8 6 L 18 6 C 19 6 20 7 20 8 Z M 16 2 L 6 2 C 5 2 4 3 4 4 L 4 14",

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
