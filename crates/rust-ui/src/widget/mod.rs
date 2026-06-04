//! Built-in widgets.
//!
//! Every widget is a plain Rust value (no `Box<dyn>` required at the call site).
//! Widgets implement the `Widget` trait which gives them layout + draw + event handling.

pub mod button;
pub mod column;
pub mod container;
pub mod input;
pub mod row;
pub mod switch;
pub mod text;

pub use button::{button, Button};
pub use column::{column, Column};
pub use container::{container, Container};
pub use input::{input, Input};
pub use row::{row, Row};
pub use switch::{switch, Switch};
pub use text::{text, Text};

use crate::event::{Event, EventStatus};
use crate::render::{Rect, Renderer};
use crate::style::Theme;

/// The core widget trait.
///
/// Every widget must implement `draw`. Layout and event handling are optional —
/// leaf widgets (Text, Button) override `handle_event`; container widgets
/// override `children` and `layout_style`.
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
}
