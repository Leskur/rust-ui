use std::cell::RefCell;
use std::rc::Rc;

use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{radio_group, radio_group_options, RadioGroupOption, Widget};

pub fn page() -> Box<dyn Widget> {
    let selected = Rc::new(RefCell::new("comfortable".to_string()));

    Box::new(
        rust_ui::column![
            text("Radio Group").size(22.0).bold(),
            text("A set of checkable buttons where only one can be checked at a time.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            radio_group(
                vec![
                    ("default", "Default"),
                    ("comfortable", "Comfortable"),
                    ("compact", "Compact"),
                ],
                selected.clone(),
            )
            .on_change(|v| println!("radio: {v}")),
            section_label("DISABLED OPTION"),
            radio_group_options(
                vec![
                    RadioGroupOption::new("free", "Free"),
                    RadioGroupOption::new("pro", "Pro"),
                    RadioGroupOption::new("enterprise", "Enterprise").disabled(true),
                ],
                Rc::new(RefCell::new("free".to_string())),
            ),
            text("Click an option or focus the group and use arrow keys + Space.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            code_block(
                r#"
let selected = Rc::new(RefCell::new("comfortable".to_string()));

radio_group(
    vec![
        ("default", "Default"),
        ("comfortable", "Comfortable"),
        ("compact", "Compact"),
    ],
    selected.clone(),
)
.on_change(|value| println!("{value}"))
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
