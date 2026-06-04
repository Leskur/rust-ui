//! Widget gallery — showcases all built-in widgets.
//!
//! ```
//! cargo run -p gallery
//! ```

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::{button, container, input, switch, text};

fn main() {
    let theme = Theme::dark();

    let gallery = rust_ui::column![
        // ── Header ───────────────────────────────────────────────────────
        text("rust-ui Gallery").size(28.0).bold(),
        text("All built-in widgets").size(14.0).color(Color::hex("#8b8fa8")),

        // ── Buttons ──────────────────────────────────────────────────────
        text("Buttons").size(18.0).bold(),
        rust_ui::row![
            button("Primary").variant(ButtonVariant::Primary),
            button("Secondary").variant(ButtonVariant::Secondary),
            button("Danger").variant(ButtonVariant::Danger),
            button("Ghost").variant(ButtonVariant::Ghost),
        ]
        .spacing(8.0),

        // ── Switch ───────────────────────────────────────────────────────
        text("Switch").size(18.0).bold(),
        rust_ui::row![
            switch("System proxy", true),
            switch("Dark mode", false),
        ]
        .spacing(24.0),

        // ── Input ────────────────────────────────────────────────────────
        text("Input").size(18.0).bold(),
        input("").placeholder("Type something…"),

        // ── Cards ────────────────────────────────────────────────────────
        text("Container / Card").size(18.0).bold(),
        rust_ui::row![
            container(
                rust_ui::column![
                    text("Card Title").size(16.0).bold(),
                    text("Some description text goes here.").size(13.0).color(Color::hex("#8b8fa8")),
                ].spacing(6.0)
            )
            .bg(Color::hex("#161820"))
            .radius(12.0)
            .padding(16.0)
            .border(Color::hex("#2a2d3e"), 1.0),

            container(
                rust_ui::column![
                    text("Accent Card").size(16.0).bold(),
                    text("Highlighted with accent border.").size(13.0).color(Color::hex("#8b8fa8")),
                ].spacing(6.0)
            )
            .bg(Color::hex("#161820"))
            .radius(12.0)
            .padding(16.0)
            .border(Color::hex("#5c7cfa"), 1.5),
        ]
        .spacing(12.0),
    ]
    .spacing(20.0)
    .padding(32.0);

    rust_ui_wgpu::run("rust-ui Gallery", 800, 600, gallery, theme);
}
