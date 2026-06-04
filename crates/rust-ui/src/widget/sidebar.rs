//! Sidebar navigation widget.
//!
//! Three-level structure mirroring shadcn/ui Sidebar:
//! - `Sidebar`       — outer container (fixed width, scrollable)
//! - `SidebarGroup`  — labelled section with multiple items
//! - `SidebarItem`   — single clickable menu entry
//!
//! ```rust,ignore
//! sidebar(vec![
//!     sidebar_group("Components", vec![
//!         sidebar_item("Button"),
//!         sidebar_item("Switch"),
//!         sidebar_item("Input"),
//!     ]),
//! ])
//! .width(220.0)
//! .active("Button")
//! .on_select(|name| println!("selected: {name}"))
//! ```

use crate::color::Color;
use crate::event::{Event, EventStatus, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

// ── SidebarItem ───────────────────────────────────────────────────────────────

pub struct SidebarItem {
    id:       String,
    label:    String,
    hovered:  bool,
    /// Set externally by Sidebar when this item matches the active key.
    pub active:   bool,
    on_click: Option<Box<dyn Fn(&str)>>,
}

impl SidebarItem {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            id:       format!("sidebar-item-{}", uuid()),
            label,
            hovered:  false,
            active:   false,
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn label(&self) -> &str { &self.label }

    fn draw_at(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let bg = if self.active {
            theme.bg_elevated
        } else if self.hovered {
            theme.bg_elevated.with_alpha(0.5)
        } else {
            Color::TRANSPARENT
        };

        if bg.a > 0.01 {
            renderer.fill_rect(bounds, bg, Corners::all(6.0));
        }

        // Active indicator bar on left edge
        if self.active {
            renderer.fill_rect(
                Rect::new(bounds.x, bounds.y + 4.0, 3.0, bounds.height - 8.0),
                theme.accent,
                Corners::all(2.0),
            );
        }

        let text_color = if self.active {
            theme.fg
        } else if self.hovered {
            theme.fg
        } else {
            theme.fg_muted
        };

        let opts = TextOptions {
            font_size: theme.font_size_md,
            color:     text_color,
            ..Default::default()
        };

        renderer.draw_text(
            &self.label,
            Point::new(bounds.x + 14.0, bounds.y + (bounds.height - theme.font_size_md) / 2.0),
            &opts,
        );
    }
}

// ── SidebarGroup ─────────────────────────────────────────────────────────────

pub struct SidebarGroup {
    label: Option<String>,
    items: Vec<SidebarItem>,
}

impl SidebarGroup {
    pub fn new(label: impl Into<String>, items: Vec<SidebarItem>) -> Self {
        Self { label: Some(label.into()), items }
    }

    pub fn unlabelled(items: Vec<SidebarItem>) -> Self {
        Self { label: None, items }
    }

}

const ITEM_H: f32 = 34.0;
const GROUP_LABEL_H: f32 = 28.0; // font_size_sm(12) + 16 padding

// ── Sidebar ───────────────────────────────────────────────────────────────────

pub struct Sidebar {
    id:        String,
    groups:    Vec<SidebarGroup>,
    width:     f32,
    active:    String,
    on_select: Option<Box<dyn Fn(&str)>>,
    cursor_pos: Point,
}

impl Sidebar {
    pub fn new(groups: Vec<SidebarGroup>) -> Self {
        Self {
            id:        format!("sidebar-{}", uuid()),
            groups,
            width:     220.0,
            active:    String::new(),
            on_select: None,
            cursor_pos: Point::new(-1.0, -1.0),
        }
    }

    pub fn width(mut self, w: f32) -> Self { self.width = w; self }

    pub fn active(mut self, key: impl Into<String>) -> Self {
        self.active = key.into(); self
    }

    pub fn on_select(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_select = Some(Box::new(f)); self
    }

    /// Iterate all items with their y-offsets relative to sidebar top.
    /// Uses fixed constants so it can be called without a Theme reference.
    fn item_rects(&self, base_x: f32, base_y: f32) -> Vec<(usize, usize, Rect)> {
        let mut out = Vec::new();
        let mut y = base_y + 8.0;
        let item_w = self.width - 16.0;

        for (gi, group) in self.groups.iter().enumerate() {
            if group.label.is_some() {
                y += GROUP_LABEL_H;
            } else {
                y += 8.0;
            }
            for (ii, _item) in group.items.iter().enumerate() {
                out.push((gi, ii, Rect::new(base_x + 8.0, y, item_w, ITEM_H)));
                y += ITEM_H;
            }
            y += 8.0;
        }
        out
    }
}

impl Widget for Sidebar {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        // Background panel
        renderer.fill_rect(
            Rect::new(bounds.x, bounds.y, self.width, bounds.height),
            theme.bg_surface,
            Corners::ZERO,
        );
        // Right border separator
        renderer.fill_rect(
            Rect::new(bounds.x + self.width - 1.0, bounds.y, 1.0, bounds.height),
            theme.border,
            Corners::ZERO,
        );

        let mut y = bounds.y + 8.0;

        for group in &self.groups {
            // Group label
            if let Some(label) = &group.label {
                y += 8.0;
                let opts = TextOptions {
                    font_size: theme.font_size_sm,
                    color:     theme.fg_subtle,
                    ..Default::default()
                };
                renderer.draw_text(label, Point::new(bounds.x + 14.0, y), &opts);
                y += GROUP_LABEL_H - 8.0;
            } else {
                y += 8.0;
            }

            for item in &group.items {
                let is_active = item.label == self.active;
                let item_bounds = Rect::new(bounds.x + 8.0, y, self.width - 16.0, ITEM_H);

                // Inline draw so we can override active flag without mut
                let bg = if is_active {
                    theme.bg_elevated
                } else if item.hovered {
                    theme.bg_elevated.with_alpha(0.5)
                } else {
                    Color::TRANSPARENT
                };

                if bg.a > 0.01 {
                    renderer.fill_rect(item_bounds, bg, Corners::all(6.0));
                }
                if is_active {
                    renderer.fill_rect(
                        Rect::new(item_bounds.x, item_bounds.y + 4.0, 3.0, item_bounds.height - 8.0),
                        theme.accent,
                        Corners::all(2.0),
                    );
                }

                let text_color = if is_active || item.hovered { theme.fg } else { theme.fg_muted };
                let opts = TextOptions {
                    font_size: theme.font_size_md,
                    color:     text_color,
                    ..Default::default()
                };
                renderer.draw_text(
                    &item.label,
                    Point::new(item_bounds.x + 14.0, item_bounds.y + (ITEM_H - theme.font_size_md) / 2.0),
                    &opts,
                );

                y += ITEM_H;
            }
            y += 8.0;
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        match event {
            Event::MouseMove { pos } => {
                self.cursor_pos = *pos;
                let rects = self.item_rects(bounds.x, bounds.y);
                for (gi, ii, rect) in &rects {
                    self.groups[*gi].items[*ii].hovered = rect.contains(pos.x, pos.y);
                }
                EventStatus::Ignored
            }
            Event::MouseClick { pos, button: MouseButton::Left } => {
                let rects = self.item_rects(bounds.x, bounds.y);
                for (gi, ii, rect) in &rects {
                    if rect.contains(pos.x, pos.y) {
                        let label = self.groups[*gi].items[*ii].label.clone();
                        self.active = label.clone();
                        if let Some(f) = &self.on_select { f(&label); }
                        return EventStatus::Consumed;
                    }
                }
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }
}

// ── Constructors ──────────────────────────────────────────────────────────────

pub fn sidebar(groups: Vec<SidebarGroup>) -> Sidebar {
    Sidebar::new(groups)
}

pub fn sidebar_group(label: impl Into<String>, items: Vec<SidebarItem>) -> SidebarGroup {
    SidebarGroup::new(label, items)
}

pub fn sidebar_item(label: impl Into<String>) -> SidebarItem {
    SidebarItem::new(label)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("{}", CTR.fetch_add(1, Ordering::Relaxed))
}
