use rust_ui::prelude::*;
use rust_ui::widget::code_block::code_block;
use rust_ui::widget::{select, select_entries, SelectEntry, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Select").size(22.0).bold(),
            text("Displays a list of options for the user to pick from—triggered by a button.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            // ── Basic (shadcn: Select a fruit) ────────────────────────────────
            example_section(
                "Basic",
                "Select a fruit",
                select(vec!["Apple", "Banana", "Blueberry", "Grapes", "Pineapple"])
                    .placeholder("Select a fruit")
                    .width(180.0),
            ),
            // ── Groups (shadcn: SelectGroup + SelectLabel + SelectSeparator) ──
            example_section(
                "Groups",
                "Select a fruit",
                select_entries(vec![
                    SelectEntry::label("Fruits"),
                    SelectEntry::item("Apple"),
                    SelectEntry::item("Banana"),
                    SelectEntry::item("Blueberry"),
                    SelectEntry::item("Grapes"),
                    SelectEntry::item("Pineapple"),
                    SelectEntry::separator(),
                    SelectEntry::label("Vegetables"),
                    SelectEntry::item("Aubergine"),
                    SelectEntry::item("Broccoli"),
                    SelectEntry::item("Carrot"),
                    SelectEntry::item("Courgette"),
                ])
                .placeholder("Select a fruit")
                .width(200.0),
            ),
            // ── Scrollable (shadcn: Select a timezone) ──────────────────────
            example_section(
                "Scrollable",
                "Select a timezone",
                select(vec![
                    "UTC",
                    "America/New_York",
                    "America/Chicago",
                    "America/Denver",
                    "America/Los_Angeles",
                    "America/Anchorage",
                    "Pacific/Honolulu",
                    "Europe/London",
                    "Europe/Paris",
                    "Europe/Berlin",
                    "Europe/Moscow",
                    "Asia/Dubai",
                    "Asia/Kolkata",
                    "Asia/Shanghai",
                    "Asia/Tokyo",
                    "Asia/Seoul",
                    "Australia/Sydney",
                    "Pacific/Auckland",
                ])
                .placeholder("Select a timezone")
                .width(220.0),
            ),
            // ── Disabled ────────────────────────────────────────────────────
            example_section(
                "Disabled",
                "Select a fruit",
                select(vec!["Apple", "Banana", "Blueberry"])
                    .placeholder("Select a fruit")
                    .width(180.0)
                    .disabled(true),
            ),
            // ── Invalid (shadcn: Field data-invalid) ────────────────────────
            invalid_field_example(),
            code_block(
                r#"
// Basic
select(vec!["Apple", "Banana", "Cherry"])
    .placeholder("Select a fruit")
    .width(180.0)

// Groups
select_entries(vec![
    SelectEntry::label("Fruits"),
    SelectEntry::item("Apple"),
    SelectEntry::separator(),
    SelectEntry::label("Vegetables"),
    SelectEntry::item("Carrot"),
])
        "#,
            ),
            section_label("KEYBOARD"),
            text("Tab to focus · Space/Enter to open · ↑/↓ to navigate · Esc to close")
                .size(12.0)
                .color(Color::hex("#8b8fa8")),
        ]
        .spacing(20.0),
    )
}

/// Mimics shadcn example blocks: section title + demo row + caption.
fn example_section(
    title: &str,
    caption: &str,
    widget: rust_ui::widget::Select,
) -> rust_ui::widget::Column {
    rust_ui::column![
        section_label(title),
        rust_ui::row![widget,].spacing(0.0),
        text(caption)
            .size(12.0)
            .color(Color::hex("#8b8fa8")),
    ]
    .spacing(8.0)
}

/// Mimics shadcn invalid Field: label + select + error message.
fn invalid_field_example() -> rust_ui::widget::Column {
    rust_ui::column![
        section_label("Invalid"),
        rust_ui::column![
            text("Fruit").size(13.0).color(Color::hex("#e2e4ec")),
            select(vec!["Apple", "Banana", "Blueberry"])
                .placeholder("Select a fruit")
                .width(180.0)
                .invalid(true),
            text("Please select a fruit.")
                .size(12.0)
                .color(Color::hex("#f87171")),
        ]
        .spacing(6.0),
    ]
    .spacing(8.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
