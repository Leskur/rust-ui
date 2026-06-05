use rust_ui::prelude::*;
use rust_ui::widget::{button, divider, spacer, text, Alignment, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Layout Primitives").size(22.0).bold(),
            text("Spacer, Stack, and Divider.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("SPACER"),
            text("Fills remaining space in a Row.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            rust_ui::row![button("Left"), spacer(), button("Right"),],
            rust_ui::row![button("A"), spacer(), button("B"), spacer(), button("C"),],
            rust_ui::row![button("Confirm"), spacer().size(32.0), button("Cancel"),],
            section_label("DIVIDER"),
            divider(),
            divider().thickness(2.0).margin(8.0),
            rust_ui::row![
                text("Left").size(13.0),
                spacer().size(12.0),
                divider().vertical(),
                spacer().size(12.0),
                text("Right").size(13.0),
            ],
            section_label("STACK"),
            text("Children share bounds, drawn back-to-front.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            rust_ui::stack![
                rust_ui::widget::container::container(rust_ui::widget::text::text(""))
                    .bg(Color::hex("#1e2030"))
                    .radius(8.0)
                    .padding(48.0),
                rust_ui::widget::stack::stack(vec![
                    Box::new(text("Overlaid text").size(14.0).bold()) as Box<dyn Widget>,
                ])
                .alignment(Alignment::Center),
            ],
        ]
        .spacing(14.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
