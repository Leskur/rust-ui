use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{progress, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Progress").size(22.0).bold(),
            text("Displays an indicator showing the completion progress of a task.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("DETERMINATE"),
            rust_ui::column![
                labeled("33%", progress().value(33.0).width(320.0)),
                labeled("66%", progress().value(66.0).width(320.0)),
                labeled("100%", progress().value(100.0).width(320.0)),
            ]
            .spacing(12.0),
            section_label("INDETERMINATE"),
            labeled(
                "Loading…",
                progress().indeterminate().width(320.0),
            ),
            section_label("SIZES"),
            rust_ui::column![
                labeled("Default (8px)", progress().value(50.0).width(320.0)),
                labeled("Thin (4px)", progress().value(50.0).width(320.0).height(4.0)),
            ]
            .spacing(12.0),
            code_block(
                r#"
progress().value(33.0).width(320.0);
progress().indeterminate().width(320.0);
        "#,
            ),
        ]
        .spacing(20.0),
    )
}

fn labeled(label: &str, bar: rust_ui::widget::progress::Progress) -> rust_ui::widget::Column {
    rust_ui::column![
        text(label).size(12.0).color(Color::hex("#8b8fa8")),
        bar,
    ]
    .spacing(6.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
