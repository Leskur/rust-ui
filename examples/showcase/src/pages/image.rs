use rust_ui::prelude::*;
use rust_ui::widget::image::{image, ObjectFit};
use rust_ui::widget::{text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(
        rust_ui::column![
            text("Image").size(22.0).bold(),
            text("Local files, remote URLs, ObjectFit modes — async loading with cache.")
                .size(13.0)
                .color(Color::hex("#8b8fa8")),
            section_label("NETWORK — ObjectFit::Cover"),
            rust_ui::row![
                image("https://picsum.photos/seed/a/400/300")
                    .width(180.0)
                    .height(120.0)
                    .fit(ObjectFit::Cover)
                    .radius(8.0),
                image("https://picsum.photos/seed/b/400/300")
                    .width(180.0)
                    .height(120.0)
                    .fit(ObjectFit::Cover)
                    .radius(8.0),
                image("https://picsum.photos/seed/c/400/300")
                    .width(180.0)
                    .height(120.0)
                    .fit(ObjectFit::Cover)
                    .radius(8.0),
            ]
            .spacing(12.0),
            section_label("NETWORK — ObjectFit::Contain"),
            rust_ui::row![
                image("https://picsum.photos/seed/d/300/400")
                    .width(120.0)
                    .height(160.0)
                    .fit(ObjectFit::Contain)
                    .radius(4.0),
                image("https://picsum.photos/seed/e/300/400")
                    .width(120.0)
                    .height(160.0)
                    .fit(ObjectFit::Contain)
                    .radius(4.0),
            ]
            .spacing(12.0),
            section_label("NETWORK — ObjectFit::Fill (stretch)"),
            image("https://picsum.photos/seed/f/800/200")
                .width(500.0)
                .height(80.0)
                .fit(ObjectFit::Fill)
                .radius(6.0),
            section_label("ERROR STATE"),
            image("https://this-domain-does-not-exist.invalid/404.png")
                .width(120.0)
                .height(80.0)
                .radius(4.0),
        ]
        .spacing(16.0),
    )
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s)
        .size(11.0)
        .color(Color::hex("#555870"))
}
