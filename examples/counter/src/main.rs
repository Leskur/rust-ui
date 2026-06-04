//! Counter example — the "Hello World" of UI frameworks.
//!
//! ```
//! cargo run -p counter
//! ```

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;

fn main() {
    let theme = Theme::dark();

    let ui = rust_ui::column![
        text("Counter").size(24.0).bold(),
        text("0").size(48.0).color(Color::hex("#5c7cfa")),
        rust_ui::row![
            button("  −  ").variant(ButtonVariant::Secondary),
            button("  +  ").variant(ButtonVariant::Primary),
        ]
        .spacing(8.0),
    ]
    .spacing(16.0)
    .padding(32.0);

    rust_ui_wgpu::run("Counter — rust-ui", 480, 320, ui, theme);
}
