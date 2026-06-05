use std::cell::RefCell;
use std::rc::Rc;

use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, dialog, input, Widget};

pub fn page() -> Box<dyn Widget> {
    let open = Rc::new(RefCell::new(false));
    let open_btn = open.clone();
    let open_cancel = open.clone();
    let open_confirm = open.clone();

    let dlg = dialog(
        rust_ui::column![
            text("This action cannot be undone. This will permanently delete your account and remove your data from our servers.")
                .size(14.0)
                .color(Color::hex("#8b8fa8")),
            input("").placeholder("Type DELETE to confirm"),
            rust_ui::row![
                button("Cancel")
                    .variant(ButtonVariant::Secondary)
                    .on_click(move || {
                        *open_cancel.borrow_mut() = false;
                    }),
                button("Delete Account")
                    .variant(ButtonVariant::Danger)
                    .on_click(move || {
                        *open_confirm.borrow_mut() = false;
                    }),
            ]
            .spacing(8.0),
        ]
        .spacing(16.0),
    )
    .title("Are you absolutely sure?")
    .shared_open(open.clone())
    .on_close({
        let open = open.clone();
        move || {
            *open.borrow_mut() = false;
        }
    });

    let main = rust_ui::column![
        text("Dialog").size(22.0).bold(),
        text("Modal overlay with backdrop dismiss, Escape key, and focus trap.")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        section_label("BASIC"),
        button("Open Dialog").on_click(move || {
            *open_btn.borrow_mut() = true;
        }),
        code_block(
            r#"
let open = Rc::new(RefCell::new(false));

stack![
    main_content,
    dialog(column![
        text("Are you sure?"),
        row![
            button("Cancel").on_click(|| *open.borrow_mut() = false),
            button("Confirm").on_click(|| *open.borrow_mut() = false),
        ],
    ])
    .title("Confirm")
    .shared_open(open.clone()),
]
        "#,
        ),
        section_label("FEATURES"),
        text("• Click backdrop to dismiss")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        text("• Press Escape to close")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
        text("• Tab stays inside the dialog (focus trap)")
            .size(13.0)
            .color(Color::hex("#8b8fa8")),
    ]
    .spacing(14.0);

    Box::new(rust_ui::stack![main, dlg])
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
