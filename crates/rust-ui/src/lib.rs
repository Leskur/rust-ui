//! # rust-ui
//!
//! A beautiful, frontend-friendly UI library for Rust desktop applications.
//!
//! ## Design goals
//! - **Familiar API** — builder-style widget API inspired by SwiftUI and CSS
//! - **Renderer-agnostic** — core has zero GPU dependencies; plug in any backend
//! - **Composable** — every widget is a value, styles compose like CSS classes
//! - **Animated** — first-class animation scheduler, no boilerplate
//!
//! ## Quick start
//! ```rust,ignore
//! use rust_ui::prelude::*;
//!
//! let ui = column![
//!     text("Hello, rust-ui!").size(24.0).color(Color::WHITE),
//!     button("Click me").on_click(|| println!("clicked!")),
//! ]
//! .spacing(12.0)
//! .padding(24.0);
//! ```

pub mod animation;
pub mod color;
pub mod event;
pub mod layout;
pub mod render;
pub mod style;
pub mod widget;

pub mod prelude {
    pub use crate::color::Color;
    pub use crate::style::{Style, Theme};
    pub use crate::widget::{button, column, row, text, Button, Column, Row, Text};
}
