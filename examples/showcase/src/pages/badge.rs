use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{badge, button, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Badge").size(22.0).bold(),
            text("Displays a badge or a component that looks like a badge.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("VARIANTS"),
            rust_ui::row![
                badge("Default"),
                badge("Secondary").variant(BadgeVariant::Secondary),
                badge("Destructive").variant(BadgeVariant::Destructive),
                badge("Outline").variant(BadgeVariant::Outline),
            ]
            .spacing(8.0),
            section_label("WITH BUTTON"),
            rust_ui::row![
                button("Messages"),
                badge("12").variant(BadgeVariant::Secondary),
            ]
            .spacing(8.0),
            code_block(
                r#"
badge("New");
badge("Beta").variant(BadgeVariant::Secondary);
badge("Error").variant(BadgeVariant::Destructive);
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
