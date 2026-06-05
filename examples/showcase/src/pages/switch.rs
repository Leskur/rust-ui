use rust_ui::prelude::*;
use rust_ui::widget::{switch, text, SwitchSize, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(rust_ui::column![
        text("Switch").size(22.0).bold(),
        text("Animated toggle with on/off state.")
            .size(13.0).color(Color::hex("#8b8fa8")),

        section_label("BASIC"),
        switch("System Proxy",   true),
        switch("Dark Mode",      true),
        switch("Notifications",  false),
        switch("Auto Update",    false),

        section_label("SIZE"),
        rust_ui::row![
            rust_ui::column![
                switch("Small", true).size(SwitchSize::Sm),
                text("Sm").size(11.0).color(Color::hex("#555870")),
            ].spacing(4.0),
            rust_ui::column![
                switch("Medium (default)", true).size(SwitchSize::Md),
                text("Md").size(11.0).color(Color::hex("#555870")),
            ].spacing(4.0),
            rust_ui::column![
                switch("Large", true).size(SwitchSize::Lg),
                text("Lg").size(11.0).color(Color::hex("#555870")),
            ].spacing(4.0),
        ].spacing(16.0),

        section_label("STATES"),
        rust_ui::row![
            rust_ui::column![
                switch("Normal", true),
                text("Normal").size(11.0).color(Color::hex("#555870")),
            ].spacing(4.0),
            rust_ui::column![
                switch("Disabled", true).disabled(true),
                text("Disabled").size(11.0).color(Color::hex("#555870")),
            ].spacing(4.0),
        ].spacing(16.0),
    ].spacing(14.0))
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s).size(11.0).color(Color::hex("#555870"))
}
