use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{slider, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Slider").size(22.0).bold(),
            text("An input where the user selects a value from within a given range.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            slider(50.0)
                .width(280.0)
                .show_value(true)
                .on_change(|v| println!("slider: {v}")),
            section_label("RANGE"),
            rust_ui::column![
                labeled(
                    "0 – 100 (default)",
                    slider(33.0).width(280.0).show_value(true),
                ),
                labeled(
                    "Volume 0 – 1, step 0.1",
                    slider(0.5)
                        .range(0.0, 1.0)
                        .step(0.1)
                        .width(280.0)
                        .show_value(true),
                ),
            ]
            .spacing(14.0),
            section_label("STATES"),
            labeled("Disabled", slider(40.0).width(280.0).disabled(true)),
            text("Drag the thumb or focus and use ← → Home End keys.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            code_block(
                r#"
slider(50.0)
    .range(0.0, 100.0)
    .step(1.0)
    .width(280.0)
    .show_value(true)
    .on_change(|v| println!("{v}"))
        "#,
            ),
        ]
        .spacing(20.0),
    )
}

fn labeled(label: &str, control: rust_ui::widget::slider::Slider) -> rust_ui::widget::Column {
    rust_ui::column![
        text(label).size(12.0).color(Color::hex("#8b8fa8")),
        control,
    ]
    .spacing(6.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
