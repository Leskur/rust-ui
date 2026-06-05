use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, tooltip, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Tooltip").size(22.0).bold(),
            text("A popup that displays information related to an element when it receives focus or is hovered.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            rust_ui::row![
                tooltip(
                    button("Hover me").variant(ButtonVariant::Outline),
                    "Add to library",
                ),
                tooltip(
                    button("Top").variant(ButtonVariant::Secondary),
                    "Tooltip on top",
                )
                .side(PopoverSide::Top),
                tooltip(
                    button("Right").variant(ButtonVariant::Secondary),
                    "Tooltip on the right",
                )
                .side(PopoverSide::Right),
            ]
            .spacing(12.0),
            section_label("DELAY"),
            rust_ui::row![
                tooltip(button("Default (400ms)").variant(ButtonVariant::Outline), "Shows after 400ms"),
                tooltip(
                    button("Instant").variant(ButtonVariant::Outline),
                    "No delay",
                )
                .delay_ms(0),
                tooltip(
                    button("Slow (800ms)").variant(ButtonVariant::Outline),
                    "Shows after 800ms",
                )
                .delay_ms(800),
            ]
            .spacing(12.0),
            section_label("DISABLED"),
            tooltip(
                button("Disabled trigger").variant(ButtonVariant::Outline).disabled(true),
                "You won't see this",
            ),
            code_block(
                r#"
tooltip(button("Hover me"), "Add to library")
    .side(PopoverSide::Top)
    .delay_ms(400)
        "#,
            ),
        ]
        .spacing(20.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
