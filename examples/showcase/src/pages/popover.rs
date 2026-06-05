use std::cell::RefCell;
use std::rc::Rc;

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, input, popover, Widget};

pub fn page() -> Box<dyn Widget> {
    let open = Rc::new(RefCell::new(false));
    let toggle = open.clone();

    Box::new(
        rust_ui::column![
            text("Popover").size(22.0).bold(),
            text("Displays rich content in a portal, triggered by a button.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            popover(
                button("Open popover")
                    .variant(ButtonVariant::Outline)
                    .on_click(move || {
                        *toggle.borrow_mut() = !*toggle.borrow_mut();
                    }),
                rust_ui::column![
                    text("Dimensions").size(14.0).bold(),
                    text("Set the dimensions for the layer.")
                        .size(12.0)
                        .color(Color::hex("#8b8fa8")),
                    rust_ui::row![
                        rust_ui::column![
                            text("Width").size(12.0).color(Color::hex("#8b8fa8")),
                            input("100%").width(120.0),
                        ]
                        .spacing(4.0),
                        rust_ui::column![
                            text("Max. Width").size(12.0).color(Color::hex("#8b8fa8")),
                            input("300px").width(120.0),
                        ]
                        .spacing(4.0),
                    ]
                    .spacing(12.0),
                    rust_ui::column![
                        text("Height").size(12.0).color(Color::hex("#8b8fa8")),
                        input("25px").width(260.0),
                    ]
                    .spacing(4.0),
                    rust_ui::column![
                        text("Max. Height").size(12.0).color(Color::hex("#8b8fa8")),
                        input("none").width(260.0),
                    ]
                    .spacing(4.0),
                ]
                .spacing(12.0),
            )
            .shared_open(open)
            .width(320.0),
            text("Click the trigger to open. Click outside or press Esc to close.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            code_block(
                r#"
let open = Rc::new(RefCell::new(false));

popover(
    button("Open popover").on_click({ let o = open.clone(); move || *o.borrow_mut() = true }),
    column![
        text("Dimensions").size(14.0).bold(),
        text("Set the dimensions for the layer."),
    ],
)
.shared_open(open)
.width(320.0)
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
