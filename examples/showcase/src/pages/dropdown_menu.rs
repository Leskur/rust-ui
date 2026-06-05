use rust_ui::prelude::*;
use rust_ui::widget::button::ButtonVariant;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{button, dropdown_menu, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Dropdown Menu").size(22.0).bold(),
            text("Displays a menu of actions triggered by a button.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            dropdown_menu(
                button("Open menu").variant(ButtonVariant::Outline),
                vec![
                    SelectEntry::item("Profile"),
                    SelectEntry::item("Billing"),
                    SelectEntry::item("Settings"),
                    SelectEntry::separator(),
                    SelectEntry::item("Sign out"),
                ],
            )
            .width(200.0)
            .on_select(|label| println!("dropdown: {label}")),
            text("Click the trigger or focus it and press Enter. Arrow keys navigate items.")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
            section_label("GROUPS"),
            dropdown_menu(
                button("Actions").variant(ButtonVariant::Secondary),
                vec![
                    SelectEntry::label("Edit"),
                    SelectEntry::item("Copy"),
                    SelectEntry::item("Paste"),
                    SelectEntry::separator(),
                    SelectEntry::label("Danger"),
                    SelectEntry::item_disabled("Delete"),
                ],
            )
            .width(180.0),
            code_block(
                r#"
dropdown_menu(
    button("Open menu"),
    vec![
        SelectEntry::item("Profile"),
        SelectEntry::separator(),
        SelectEntry::item("Sign out"),
    ],
)
.on_select(|label| println!("{label}"))
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
