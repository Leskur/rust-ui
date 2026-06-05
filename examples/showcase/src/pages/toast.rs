use std::cell::RefCell;
use std::rc::Rc;

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, Widget};

pub fn page(store: Rc<RefCell<ToastStore>>) -> Box<dyn Widget> {
    let show = store.clone();
    let success = store.clone();
    let error = store.clone();
    let desc = store.clone();
    let stack = store.clone();

    Box::new(
        rust_ui::column![
            text("Toast").size(22.0).bold(),
            text("Stacked notifications with auto-dismiss and manual close (Sonner-style).")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            rust_ui::row![
                button("Show toast")
                    .variant(ButtonVariant::Outline)
                    .on_click(move || {
                        show.borrow_mut().show("Event has been created");
                    }),
                button("With description")
                    .variant(ButtonVariant::Outline)
                    .on_click(move || {
                        desc.borrow_mut().message(
                            "Scheduled: Catch up",
                            "Friday, February 10, 2024 at 5:57 PM",
                        );
                    }),
            ]
            .spacing(8.0),
            section_label("VARIANTS"),
            rust_ui::row![
                button("Success")
                    .variant(ButtonVariant::Secondary)
                    .on_click(move || {
                        success.borrow_mut().success("Changes saved");
                    }),
                button("Error")
                    .variant(ButtonVariant::Secondary)
                    .on_click(move || {
                        error.borrow_mut().error("Something went wrong");
                    }),
                button("Stack three")
                    .variant(ButtonVariant::Secondary)
                    .on_click(move || {
                        let mut s = stack.borrow_mut();
                        s.show("First toast");
                        s.show("Second toast");
                        s.success("Third toast");
                    }),
            ]
            .spacing(8.0),
            text("Toasts appear bottom-right. Click × to dismiss. Auto-close after 4 seconds.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            code_block(
                r#"
let store = toast_store();

stack![
    app_content,
    toaster(store.clone()),
];

button("Save").on_click({
    let store = store.clone();
    move || store.borrow_mut().success("Saved")
})
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
