use rust_ui::prelude::*;
use rust_ui::widget::button::{ButtonSize, ButtonVariant};
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Button").size(22.0).bold(),
            text("Variants, sizes, states — all composable.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("VARIANTS"),
            rust_ui::row![
                button("Primary").variant(ButtonVariant::Primary),
                button("Secondary").variant(ButtonVariant::Secondary),
                button("Outline").variant(ButtonVariant::Outline),
                button("Danger").variant(ButtonVariant::Danger),
                button("Ghost").variant(ButtonVariant::Ghost),
            ]
            .spacing(8.0),
            code_block(
                r#"
button("Primary").variant(ButtonVariant::Primary)
button("Secondary").variant(ButtonVariant::Secondary)
button("Outline").variant(ButtonVariant::Outline)
button("Danger").variant(ButtonVariant::Danger)
button("Ghost").variant(ButtonVariant::Ghost)
        "#
            ),
            section_label("SIZES"),
            rust_ui::row![
                button("XS")
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Xs),
                button("SM")
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Sm),
                button("MD")
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Md),
                button("LG")
                    .variant(ButtonVariant::Secondary)
                    .size(ButtonSize::Lg),
            ]
            .spacing(8.0),
            code_block(
                r#"
button("XS").variant(ButtonVariant::Secondary).size(ButtonSize::Xs)
button("SM").variant(ButtonVariant::Secondary).size(ButtonSize::Sm)
button("MD").variant(ButtonVariant::Secondary).size(ButtonSize::Md)
button("LG").variant(ButtonVariant::Secondary).size(ButtonSize::Lg)
        "#
            ),
            section_label("STATES"),
            rust_ui::row![
                button("Disabled")
                    .variant(ButtonVariant::Primary)
                    .disabled(true),
                button("Disabled")
                    .variant(ButtonVariant::Secondary)
                    .disabled(true),
                button("Loading")
                    .variant(ButtonVariant::Primary)
                    .loading(true),
                button("Loading")
                    .variant(ButtonVariant::Outline)
                    .loading(true),
            ]
            .spacing(8.0),
            code_block(
                r#"
button("Disabled").variant(ButtonVariant::Primary).disabled(true)
button("Loading").variant(ButtonVariant::Primary).loading(true)
        "#
            ),
        ]
        .spacing(14.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
