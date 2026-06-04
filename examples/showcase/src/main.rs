//! rust-ui Showcase — interactive component browser.
//!
//! Left sidebar: component list
//! Right panel:  live demo of the selected component
//!
//! ```
//! cargo run -p showcase
//! ```

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::{
    button, container, input, sidebar, sidebar_group, sidebar_item, switch, text,
};

fn main() {
    let theme = Theme::dark();

    // ── Sidebar ───────────────────────────────────────────────────────────────
    let nav = sidebar(vec![
        sidebar_group(
            "rust-ui",
            vec![sidebar_item("Overview")],
        ),
        sidebar_group(
            "Components",
            vec![
                sidebar_item("Button"),
                sidebar_item("Text"),
                sidebar_item("Input"),
                sidebar_item("Switch"),
                sidebar_item("Container"),
                sidebar_item("Row & Column"),
            ],
        ),
        sidebar_group(
            "Coming soon",
            vec![
                sidebar_item("Badge"),
                sidebar_item("Spinner"),
                sidebar_item("Select"),
                sidebar_item("Slider"),
                sidebar_item("Dialog"),
                sidebar_item("Tabs"),
                sidebar_item("Table"),
            ],
        ),
    ])
    .width(220.0)
    .active("Button");

    // ── Content panels ────────────────────────────────────────────────────────

    // Button demo
    let button_demo = rust_ui::column![
        text("Button").size(22.0).bold(),
        text("Four semantic variants, hover and press states built in.")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        // Variants row
        rust_ui::row![
            button("Primary").variant(ButtonVariant::Primary),
            button("Secondary").variant(ButtonVariant::Secondary),
            button("Danger").variant(ButtonVariant::Danger),
            button("Ghost").variant(ButtonVariant::Ghost),
        ]
        .spacing(8.0),
        // Disabled
        text("Disabled").size(13.0).color(Color::hex("#555870")),
        rust_ui::row![
            button("Disabled").variant(ButtonVariant::Primary).disabled(true),
            button("Disabled").variant(ButtonVariant::Secondary).disabled(true),
        ]
        .spacing(8.0),
    ]
    .spacing(16.0);

    // Switch demo
    let switch_demo = rust_ui::column![
        text("Switch").size(22.0).bold(),
        text("Animated toggle with on/off state.")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        switch("System Proxy", true),
        switch("Dark Mode", false),
        switch("Notifications", true),
    ]
    .spacing(16.0);

    // Input demo
    let input_demo = rust_ui::column![
        text("Input").size(22.0).bold(),
        text("Text input with placeholder and keyboard support.")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        input("").placeholder("Search components…"),
        input("").placeholder("Enter your email"),
    ]
    .spacing(16.0);

    // Overview
    let overview = rust_ui::column![
        text("rust-ui").size(28.0).bold(),
        text("A beautiful, frontend-friendly UI library for Rust.")
            .size(15.0)
            .color(Color::hex("#8b8fa8")),
        rust_ui::row![
            container(
                rust_ui::column![
                    text("7").size(32.0).bold().color(Color::hex("#5c7cfa")),
                    text("Widgets").size(13.0).color(Color::hex("#8b8fa8")),
                ].spacing(4.0)
            )
            .bg(Color::hex("#161820"))
            .radius(12.0)
            .padding(20.0)
            .border(Color::hex("#2a2d3e"), 1.0),

            container(
                rust_ui::column![
                    text("wgpu").size(18.0).bold().color(Color::hex("#4fc08d")),
                    text("Renderer").size(13.0).color(Color::hex("#8b8fa8")),
                ].spacing(4.0)
            )
            .bg(Color::hex("#161820"))
            .radius(12.0)
            .padding(20.0)
            .border(Color::hex("#2a2d3e"), 1.0),

            container(
                rust_ui::column![
                    text("vello").size(18.0).bold().color(Color::hex("#fbbf24")),
                    text("Vector").size(13.0).color(Color::hex("#8b8fa8")),
                ].spacing(4.0)
            )
            .bg(Color::hex("#161820"))
            .radius(12.0)
            .padding(20.0)
            .border(Color::hex("#2a2d3e"), 1.0),
        ]
        .spacing(12.0),
    ]
    .spacing(20.0);

    // ── Layout: sidebar + content ─────────────────────────────────────────────
    //
    // We use Row: left = Sidebar, right = content panel.
    // (Real routing will come with the Store/dispatch system.)
    //
    let content = container(
        rust_ui::column![
            // Show button_demo as default selected content
            button_demo,
        ]
        .spacing(0.0)
    )
    .padding(32.0);

    let root = rust_ui::row![nav, content].spacing(0.0);

    rust_ui_wgpu::run("rust-ui Showcase", 1024, 680, root, theme);
}
