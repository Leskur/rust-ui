use rust_ui::prelude::*;
use rust_ui::widget::svg::svg;
use rust_ui::widget::{container, text, Widget};

fn asset(name: &str) -> String {
    format!("{}\\assets\\icons\\{}", env!("CARGO_MANIFEST_DIR"), name)
}

pub fn page() -> Box<dyn Widget> {
    Box::new(rust_ui::column![
        text("Svg").size(22.0).bold(),
        text("Load SVG from local file or URL. Rasterized by resvg.")
            .size(13.0).color(Color::hex("#8b8fa8")),

        section_label("LOCAL SVG FILES"),
        rust_ui::row![
            svg_card(&asset("chevron-left.svg"),  "chevron-left"),
            svg_card(&asset("chevron-right.svg"), "chevron-right"),
        ].spacing(12.0),

        section_label("NETWORK SVG (lucide via github)"),
        rust_ui::row![
            svg_card("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/search.svg",   "search"),
            svg_card("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/settings.svg", "settings"),
            svg_card("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/home.svg",     "home"),
            svg_card("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/heart.svg",    "heart"),
        ].spacing(12.0),

        section_label("DIFFERENT SIZES"),
        rust_ui::row![
            svg("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/star.svg").width(16.0).height(16.0),
            svg("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/star.svg").width(32.0).height(32.0),
            svg("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/star.svg").width(64.0).height(64.0),
            svg("https://raw.githubusercontent.com/lucide-icons/lucide/main/icons/star.svg").width(96.0).height(96.0),
        ].spacing(16.0),
    ].spacing(16.0))
}

fn svg_card(path: &str, label: &str) -> rust_ui::widget::container::Container {
    container(
        rust_ui::column![
            svg(path).width(48.0).height(48.0),
            text(label).size(11.0).color(Color::hex("#8b8fa8")),
        ]
    )
    .padding(16.0)
    .bg(Color::hex("#1e1e2e"))
    .radius(8.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s).size(11.0).color(Color::hex("#555870"))
}
