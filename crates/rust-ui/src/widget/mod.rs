//! Built-in widgets.
//!
//! Every widget is a plain Rust value (no `Box<dyn>` required at the call site).
//! Widgets implement the `Widget` trait which gives them layout + draw + event handling.

pub mod accordion;
pub mod button;
pub mod code_block;
pub mod badge;
pub mod checkbox;
pub mod column;
pub mod container;
pub mod dialog;
pub mod divider;
pub mod dropdown_menu;
pub mod icon;
pub mod image;
pub mod input;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod row;
pub mod scroll_area;
pub mod select;
pub mod sidebar;
pub mod slider;
pub mod spacer;
pub mod stack;
pub mod svg;
pub mod switch;
pub mod tab_view;
pub mod tabs;
pub mod text;
pub mod toast;
pub mod tooltip;

pub use accordion::{accordion, Accordion, AccordionItem};
pub use badge::{badge, Badge, BadgeVariant};
pub use button::{button, Button};
pub use code_block::{code_block, CodeBlock};
pub use checkbox::{checkbox, Checkbox};
pub use column::{column, Column};
pub use container::{container, Container};
pub use dialog::{dialog, Dialog};
pub use divider::{divider, Divider, DividerAxis};
pub use dropdown_menu::{dropdown_menu, DropdownMenu};
pub use icon::{icon, Icon};
pub use image::{image, Image, ObjectFit};
pub use input::{input, Input, InputSize};
pub use popover::{popover, Popover};
pub use progress::{progress, Progress};
pub use radio_group::{radio_group, radio_group_options, RadioGroup, RadioGroupOption};
pub use row::{row, Row};
pub use scroll_area::{scroll_area, ScrollArea};
pub use select::{select, select_entries, Select, SelectEntry};
pub use sidebar::{sidebar, sidebar_group, sidebar_item, Sidebar, SidebarGroup, SidebarItem};
pub use slider::{slider, Slider};
pub use spacer::{spacer, Spacer};
pub use stack::{stack, Alignment, Stack};
pub use svg::{svg, Svg};
pub use switch::{switch, Switch, SwitchSize};
pub use tab_view::{tab_view, TabView};
pub use tabs::{tabs, Tabs};
pub use text::{text, Text};
pub use toast::{toast_store, toaster, ToastItem, ToastPosition, ToastStore, ToastVariant, Toaster};
pub use tooltip::{tooltip, Tooltip};

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

    /// Paint an overlay layer above sibling widgets (e.g. dropdown menus, tooltips).
    ///
    /// Container widgets should call this for all children *after* drawing the
    /// normal widget tree so popups are not occluded by later siblings.
    fn draw_overlay(&self, _renderer: &mut dyn Renderer, _bounds: Rect, _theme: &Theme) {}

    /// Handle an input event. Return `Consumed` to stop propagation.
    fn handle_event(&mut self, _event: &Event, _bounds: Rect) -> EventStatus {
        EventStatus::Ignored
    }

    /// Whether this widget is a container with children.
    fn is_container(&self) -> bool {
        false
    }

    /// Return `true` while the widget is waiting on a timed interaction (e.g.
    /// tooltip hover delay) so the runtime keeps redrawing.
    fn wants_redraw(&self) -> bool {
        false
    }

    /// Walk the widget tree and return `true` if any node needs a timed redraw.
    fn poll_redraw(&self, bounds: Rect, theme: &Theme) -> bool {
        if self.wants_redraw() {
            return true;
        }
        for (child, cb) in self.layout_children(bounds, theme) {
            if child.poll_redraw(cb, theme) {
                return true;
            }
        }
        false
    }

    /// Natural (width, height) in logical pixels when unconstrained.
    /// Used by Row/Column to size children without a full layout pass.
    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
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

    /// Mutable variant of `layout_children`, used by framework features that need
    /// to traverse the widget tree and mutate a specific target (e.g. focus).
    ///
    /// Container widgets should override this alongside `layout_children`.
    fn layout_children_mut<'a>(
        &'a mut self,
        _bounds: Rect,
        _theme: &Theme,
    ) -> Vec<(&'a mut dyn Widget, Rect)> {
        vec![]
    }

    /// Whether this widget can receive keyboard focus.
    fn focusable(&self) -> bool {
        false
    }

    /// When `true`, keyboard focus and Tab navigation are confined to this
    /// widget's subtree (used by open modals / dialogs).
    fn blocks_focus_outside(&self) -> bool {
        false
    }

    /// Hit-test for pointer interactions (used by overlay-aware containers).
    ///
    /// Default behavior is `bounds.contains`.
    fn hit_test(&self, pos: (f32, f32), bounds: Rect, _theme: &Theme) -> bool {
        bounds.contains(pos.0, pos.1)
    }

    /// Return the cursor style when the pointer is at `pos` over `bounds`.
    ///
    /// **Default implementation** walks `layout_children` automatically —
    /// container widgets do NOT need to override this as long as they implement
    /// `layout_children`. Only override for custom cursor logic (e.g. scrollbars).
    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        for (child, cb) in self.layout_children(bounds, &theme) {
            if child.hit_test(pos, cb, &theme) {
                return child.cursor_at(pos, cb);
            }
        }
        CursorStyle::Default
    }

    /// Returns `true` for flexible `Spacer` instances that should expand to
    /// fill remaining space in a Row or Column layout pass.
    fn is_spacer(&self) -> bool {
        false
    }
}
