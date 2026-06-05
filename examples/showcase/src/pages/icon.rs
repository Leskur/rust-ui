use rust_ui::prelude::*;
use rust_ui::widget::svg::svg;
use rust_ui::widget::{container, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(rust_ui::column![
        text("Svg").size(22.0).bold(),
        text("Load SVG files with custom size and color.")
            .size(13.0).color(Color::hex("#8b8fa8")),

        section_label("NAVIGATION"),
        rust_ui::row![
            svg_row("assets/icons/chevron-left.svg"),
            svg_row("assets/icons/chevron-right.svg"),
        ].spacing(12.0),
    ].spacing(16.0))
}

fn svg_row(path: &str) -> rust_ui::widget::container::Container {
    container(
        rust_ui::column![
            svg(path).width(32.0).height(32.0),
            text(path).size(11.0).color(Color::hex("#8b8fa8")),
        ]
    )
    .padding(12.0)
    .bg(Color::hex("#1e1e2e"))
    .radius(8.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s).size(11.0).color(Color::hex("#555870"))
}
