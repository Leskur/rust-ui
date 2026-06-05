//! Built-in widgets.
//!
//! Every widget is a plain Rust value (no `Box<dyn>` required at the call site).
//! Widgets implement the `Widget` trait which gives them layout + draw + event handling.

pub mod button;
pub mod column;
pub mod container;
pub mod divider;
pub mod input;
pub mod row;
pub mod scroll_area;
pub mod sidebar;
pub mod spacer;
pub mod stack;
pub mod switch;
pub mod tab_view;
pub mod text;

pub use button::{button, Button};
pub use column::{column, Column};
pub use container::{container, Container};
pub use divider::{divider, Divider, DividerAxis};
pub use input::{input, Input, InputSize};
pub use row::{row, Row};
pub use scroll_area::{scroll_area, ScrollArea};
pub use sidebar::{sidebar, sidebar_group, sidebar_item, Sidebar, SidebarGroup, SidebarItem};
pub use spacer::{spacer, Spacer};
pub use stack::{stack, Stack, Alignment};
pub use switch::{switch, Switch, SwitchSize};
pub use tab_view::{tab_view, TabView};
pub use text::{text, Text};

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::{CursorStyle, Theme};

/// The core widget trait.
///
/// Every widget must implement `draw`. Layout and event handling are optional —
/// leaf widgets (Text, Button) override `handle_event`; container widgets
/// override `layout_children` to get automatic `cursor_at` propagation.
///
/// # Implementing a container widget
///
/// 1. Implement `layout_children` — return `(child_ref, child_bounds)` pairs.
///    This is the **single source of truth** for layout. The default `cursor_at`
///    will automatically walk this list, so you never need to write `cursor_at`
///    yourself in a container.
///
/// 2. Implement `handle_event` — call `child.handle_event(event, cb)` for each
///    pair from `layout_children`. Mouse events should be broadcast to *all*
///    children; keyboard events stop at the first consumer.
///
/// 3. Do **not** override `cursor_at` unless you need custom cursor logic
///    (e.g. a scrollbar thumb that shows a grab cursor).
pub trait Widget {
    /// Unique stable ID for this widget instance (used by the animation scheduler).
    fn id(&self) -> &str;

    /// Paint this widget given its computed `bounds`.
    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme);

    /// Handle an input event. Return `Consumed` to stop propagation.
    fn handle_event(&mut self, _event: &Event, _bounds: Rect) -> EventStatus {
        EventStatus::Ignored
    }

    /// Whether this widget is a container with children.
    fn is_container(&self) -> bool { false }

    /// Natural (width, height) in logical pixels when unconstrained.
    /// Used by Row/Column to size children without a full layout pass.
    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        (120.0, 36.0) // sensible default
    }

    /// Return laid-out children as `(child_ref, child_bounds)` pairs.
    ///
    /// Container widgets implement this to expose their layout to the framework.
    /// The default `cursor_at` walks this list automatically, so implementing
    /// this method is sufficient — no need to override `cursor_at` separately.
    ///
    /// The `bounds` parameter is the same bounds passed to `draw`/`handle_event`.
    fn layout_children<'a>(&'a self, _bounds: Rect, _theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        vec![]
    }

    /// Return the cursor style when the pointer is at `pos` over `bounds`.
    ///
    /// **Default implementation** walks `layout_children` automatically —
    /// container widgets do NOT need to override this as long as they implement
    /// `layout_children`. Only override for custom cursor logic (e.g. scrollbars).
    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        for (child, cb) in self.layout_children(bounds, &theme) {
            if cb.contains(pos.0, pos.1) {
                return child.cursor_at(pos, cb);
            }
        }
        CursorStyle::Default
    }

    /// Returns `true` for flexible `Spacer` instances that should expand to
    /// fill remaining space in a Row or Column layout pass.
    fn is_spacer(&self) -> bool { false }
}
