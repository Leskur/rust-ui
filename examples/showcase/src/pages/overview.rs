use rust_ui::prelude::*;
use rust_ui::widget::{container, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("rust-ui").size(28.0).bold(),
            text("A beautiful, ergonomic UI library for Rust desktop apps.")
                .size(14.0)
                .color(Color::hex("#8b8fa8")),
            rust_ui::row![
                container(
                    rust_ui::column![
                        text("8").size(32.0).bold().color(Color::hex("#5c7cfa")),
                        text("Widgets").size(12.0).color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(4.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(20.0)
                .border(Color::hex("#2a2d3e"), 1.0),
                container(
                    rust_ui::column![
                        text("wgpu").size(18.0).bold().color(Color::hex("#4fc08d")),
                        text("Renderer").size(12.0).color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(4.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(20.0)
                .border(Color::hex("#2a2d3e"), 1.0),
                container(
                    rust_ui::column![
                        text("vello").size(18.0).bold().color(Color::hex("#fbbf24")),
                        text("Vector GPU").size(12.0).color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(4.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(20.0)
                .border(Color::hex("#2a2d3e"), 1.0),
            ]
            .spacing(12.0),
        ]
        .spacing(20.0),
    )
}
