use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{accordion, AccordionItem, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Accordion").size(22.0).bold(),
            text("A vertically stacked set of interactive headings that reveal a section of content.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("SINGLE"),
            accordion(vec![
                AccordionItem::new(
                    "item-1",
                    "Is it accessible?",
                    text("Yes. It adheres to the WAI-ARIA design pattern.")
                        .size(14.0)
                        .color(Color::hex("#8b8fa8")),
                ),
                AccordionItem::new(
                    "item-2",
                    "Is it styled?",
                    text("Yes. It comes with default styles that match the other components.")
                        .size(14.0)
                        .color(Color::hex("#8b8fa8")),
                ),
                AccordionItem::new(
                    "item-3",
                    "Is it animated?",
                    text("Not yet — expand/collapse is instant in this version.")
                        .size(14.0)
                        .color(Color::hex("#8b8fa8")),
                ),
            ])
            .default_open(vec!["item-1"]),
            code_block(
                r#"
accordion(vec![
    AccordionItem::new("item-1", "Is it accessible?", text("Yes.")),
    AccordionItem::new("item-2", "Is it styled?", text("Yes.")),
])
.default_open(vec!["item-1"])
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
