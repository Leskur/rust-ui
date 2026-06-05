use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{input, tabs, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Tabs").size(22.0).bold(),
            text("A set of layered sections of content, known as tab panels, that are displayed one at a time.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            tabs(vec![
                (
                    "Account".to_string(),
                    Box::new(
                        rust_ui::column![
                            text("Account").size(16.0).bold(),
                            text("Make changes to your account here. Click save when you're done.")
                                .size(13.0)
                                .color(Color::hex("#8b8fa8")),
                            input("").placeholder("Name"),
                        ]
                        .spacing(12.0),
                    ),
                ),
                (
                    "Password".to_string(),
                    Box::new(
                        rust_ui::column![
                            text("Password").size(16.0).bold(),
                            text("Change your password here. After saving, you'll be logged out.")
                                .size(13.0)
                                .color(Color::hex("#8b8fa8")),
                            input("").placeholder("Current password"),
                            input("").placeholder("New password"),
                        ]
                        .spacing(12.0),
                    ),
                ),
            ])
            .active("Account"),
            code_block(
                r#"
tabs(vec![
    ("Account".to_string(), Box::new(account_panel)),
    ("Password".to_string(), Box::new(password_panel)),
])
.active("Account")
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
