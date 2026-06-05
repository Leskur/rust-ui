use rust_ui::prelude::*;
use rust_ui::widget::{container, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Container").size(22.0).bold(),
            text("Styled box with padding, border, radius and background.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            rust_ui::row![
                container(
                    rust_ui::column![
                        text("Default card").size(14.0).bold(),
                        text("Standard border and background.")
                            .size(12.0)
                            .color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(6.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(16.0)
                .border(Color::hex("#2a2d3e"), 1.0),
                container(
                    rust_ui::column![
                        text("Accent card").size(14.0).bold(),
                        text("Highlighted with accent border.")
                            .size(12.0)
                            .color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(6.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(16.0)
                .border(Color::hex("#5c7cfa"), 1.5),
                container(
                    rust_ui::column![
                        text("Success card").size(14.0).bold(),
                        text("Green semantic border.")
                            .size(12.0)
                            .color(Color::hex("#8b8fa8")),
                    ]
                    .spacing(6.0)
                )
                .bg(Color::hex("#161820"))
                .radius(12.0)
                .padding(16.0)
                .border(Color::hex("#4fc08d"), 1.5),
            ]
            .spacing(12.0),
        ]
        .spacing(14.0),
    )
}
