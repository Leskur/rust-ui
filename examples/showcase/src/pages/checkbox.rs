use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{checkbox, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Checkbox").size(22.0).bold(),
            text("Boolean toggle with label, focus ring, and keyboard support.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            checkbox("Remember me", true),
            checkbox("Email me product updates", false),
            section_label("STATES"),
            checkbox("Disabled (checked)", true).disabled(true),
            checkbox("Disabled (unchecked)", false).disabled(true),
            code_block(
                r#"
checkbox("Remember me", true)
checkbox("Disabled", false).disabled(true)
        "#,
            ),
            section_label("KEYBOARD"),
            text("Tip: click a checkbox then press Tab / Space / Enter.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
        ]
        .spacing(14.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}

