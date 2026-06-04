use rust_ui::prelude::*;
use rust_ui::widget::{switch, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(rust_ui::column![
        text("Switch").size(22.0).bold(),
        text("Animated toggle with on/off state.")
            .size(13.0).color(Color::hex("#8b8fa8")),
        switch("System Proxy",   true),
        switch("Dark Mode",      true),
        switch("Notifications",  false),
        switch("Auto Update",    false),
    ].spacing(14.0))
}
