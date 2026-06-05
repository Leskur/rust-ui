use rust_ui::prelude::*;
use rust_ui::widget::{input, text, InputSize, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Input").size(22.0).bold(),
            text("HTML-like editing: cursor, selection, clipboard, IME, scroll, password.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC  (click to focus, then type)"),
            rust_ui::column![
                input("").placeholder("Type something...").width(300.0),
                input("hello@example.com").placeholder("Email").width(300.0),
            ]
            .spacing(10.0),
            section_label("SIZE  (height variants)"),
            rust_ui::row![
                rust_ui::column![
                    input("")
                        .placeholder("Small")
                        .size(InputSize::Sm)
                        .width(140.0),
                    text("Sm").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    input("")
                        .placeholder("Medium (default)")
                        .size(InputSize::Md)
                        .width(180.0),
                    text("Md").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    input("")
                        .placeholder("Large")
                        .size(InputSize::Lg)
                        .width(140.0),
                    text("Lg").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
            ]
            .spacing(12.0),
            section_label("CLEARABLE"),
            rust_ui::column![
                input("").placeholder("Search...").clearable().width(300.0),
                input("pre-filled text").clearable().width(300.0),
            ]
            .spacing(10.0),
            section_label("PASSWORD"),
            rust_ui::column![
                input("")
                    .placeholder("Enter password")
                    .password()
                    .width(300.0),
                input("secret123").password().clearable().width(300.0),
            ]
            .spacing(10.0),
            section_label("STATES"),
            rust_ui::row![
                rust_ui::column![
                    input("").placeholder("Normal").width(180.0),
                    text("Normal").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    input("")
                        .placeholder("Disabled")
                        .disabled(true)
                        .width(180.0),
                    text("Disabled").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
            ]
            .spacing(16.0),
            section_label("KEYBOARD SHORTCUTS"),
            rust_ui::column![
                text("← →  move cursor      Home / End  jump to edge")
                    .size(12.0)
                    .color(Color::hex("#8b8fa8")),
                text("Shift+← →  extend selection      Ctrl+A  select all")
                    .size(12.0)
                    .color(Color::hex("#8b8fa8")),
                text("Ctrl+C  copy      Ctrl+X  cut      Ctrl+V  paste      Del  delete forward")
                    .size(12.0)
                    .color(Color::hex("#8b8fa8")),
            ]
            .spacing(4.0),
        ]
        .spacing(14.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
