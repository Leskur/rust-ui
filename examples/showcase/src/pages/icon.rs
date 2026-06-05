use rust_ui::prelude::*;
use rust_ui::widget::icon::icon;
use rust_ui::widget::{container, text, Widget};

pub fn page() -> Box<dyn Widget> {
    Box::new(rust_ui::column![
        text("Icon").size(22.0).bold(),
        text("Vector icons from lucide — scale perfectly, custom colors.")
            .size(13.0).color(Color::hex("#8b8fa8")),

        section_label("NAVIGATION"),
        rust_ui::row![
            icon_row("chevron-left"),
            icon_row("chevron-right"),
            icon_row("chevron-up"),
            icon_row("chevron-down"),
        ].spacing(12.0),

        section_label("ACTIONS"),
        rust_ui::row![
            icon_row("plus"),
            icon_row("minus"),
            icon_row("x"),
            icon_row("check"),
        ].spacing(12.0),

        section_label("UI ELEMENTS"),
        rust_ui::row![
            icon_row("search"),
            icon_row("settings"),
            icon_row("menu"),
            icon_row("bell"),
        ].spacing(12.0),

        section_label("FILES"),
        rust_ui::row![
            icon_row("folder"),
            icon_row("file"),
            icon_row("edit"),
            icon_row("copy"),
        ].spacing(12.0),

        section_label("TRANSFER"),
        rust_ui::row![
            icon_row("download"),
            icon_row("upload"),
            icon_row("external-link"),
            icon_row("trash"),
        ].spacing(12.0),

        section_label("MISCELLANEOUS"),
        rust_ui::row![
            icon_row("home"),
            icon_row("user"),
            icon_row("heart"),
            icon_row("star"),
        ].spacing(12.0),
    ].spacing(16.0))
}

fn icon_row(name: &str) -> rust_ui::widget::container::Container {
    container(
        rust_ui::column![
            icon(name).size(32.0).color(Color::hex("#6366f1")),
            text(name).size(11.0).color(Color::hex("#8b8fa8")),
        ]
    )
    .padding(12.0)
    .bg(Color::hex("#1e1e2e"))
    .radius(8.0)
}

fn section_label(s: &str) -> rust_ui::widget::text::Text {
    rust_ui::widget::text::text(s).size(11.0).color(Color::hex("#555870"))
}
