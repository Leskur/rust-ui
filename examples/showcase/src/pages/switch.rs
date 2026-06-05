use rust_ui::animation::AnimationScheduler;
use rust_ui::prelude::*;
use rust_ui::widget::{switch, text, SwitchSize, Widget};
use std::sync::{Arc, Mutex};

pub fn page(scheduler: Arc<Mutex<AnimationScheduler>>) -> Box<dyn Widget> {
    let sched = scheduler.clone();
    let sched2 = scheduler.clone();
    let sched3 = scheduler.clone();
    let sched4 = scheduler.clone();
    let sched5 = scheduler.clone();
    let sched6 = scheduler.clone();
    let sched7 = scheduler.clone();
    let sched8 = scheduler.clone();

    Box::new(
        rust_ui::column![
            text("Switch").size(22.0).bold(),
            text("Animated toggle with on/off state.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("BASIC"),
            switch("System Proxy", true).scheduler(sched.clone()),
            switch("Dark Mode", true).scheduler(sched2.clone()),
            switch("Notifications", false).scheduler(sched3.clone()),
            switch("Auto Update", false).scheduler(sched4.clone()),
            section_label("SIZE"),
            rust_ui::row![
                rust_ui::column![
                    switch("Small", true)
                        .size(SwitchSize::Sm)
                        .scheduler(sched5.clone()),
                    text("Sm").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    switch("Medium (default)", true)
                        .size(SwitchSize::Md)
                        .scheduler(sched6.clone()),
                    text("Md").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    switch("Large", true)
                        .size(SwitchSize::Lg)
                        .scheduler(sched7.clone()),
                    text("Lg").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
            ]
            .spacing(16.0),
            section_label("STATES"),
            rust_ui::row![
                rust_ui::column![
                    switch("Normal", true).scheduler(sched8.clone()),
                    text("Normal").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
                rust_ui::column![
                    switch("Disabled", true).disabled(true),
                    text("Disabled").size(11.0).color(Color::hex("#555870")),
                ]
                .spacing(4.0),
            ]
            .spacing(16.0),
        ]
        .spacing(14.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
