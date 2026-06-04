//! rust-ui Showcase — interactive component browser.
//!
//! Left sidebar: component list
//! Right panel:  live demo of the selected component
//!
//! ```
//! cargo run -p showcase
//! ```

mod pages;

use rust_ui::prelude::*;
use rust_ui::widget::{
    container, sidebar, sidebar_group, sidebar_item, tab_view, Widget,
};
use rust_ui::widget::TabView;
use rust_ui::event::{Event, EventStatus};
use rust_ui::render::{Rect, Renderer};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let theme = Theme::dark();

    // ── Content pages ─────────────────────────────────────────────────────────
    let tabs = Rc::new(RefCell::new(tab_view(vec![
        ("Overview".to_string(),  pages::overview::page()),
        ("Button".to_string(),    pages::button::page()),
        ("Input".to_string(),     pages::input::page()),
        ("Switch".to_string(),    pages::switch::page()),
        ("Container".to_string(), pages::container::page()),
    ]).active("Button")));

    // Thin wrapper so Rc<RefCell<TabView>> can be used as a Widget
    struct TabViewWrapper {
        id:    String,
        inner: Rc<RefCell<TabView>>,
    }
    impl Widget for TabViewWrapper {
        fn id(&self) -> &str { &self.id }
        fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            self.inner.borrow().draw(renderer, bounds, theme);
        }
        fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
            self.inner.borrow_mut().handle_event(event, bounds)
        }
        fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
            self.inner.borrow().intrinsic_size(theme)
        }
    }

    // ── Sidebar ───────────────────────────────────────────────────────────────
    let nav = sidebar(vec![
        sidebar_group("rust-ui", vec![sidebar_item("Overview")]),
        sidebar_group(
            "Components",
            vec![
                sidebar_item("Button"),
                sidebar_item("Input"),
                sidebar_item("Switch"),
                sidebar_item("Container"),
            ],
        ),
        sidebar_group(
            "Coming soon",
            vec![
                sidebar_item("Badge"),
                sidebar_item("Spinner"),
                sidebar_item("Select"),
                sidebar_item("Slider"),
                sidebar_item("Dialog"),
                sidebar_item("Tabs"),
                sidebar_item("Table"),
            ],
        ),
    ])
    .width(220.0)
    .active("Button")
    .on_select({
        let tabs = tabs.clone();
        move |name: &str| { tabs.borrow_mut().set_active(name); }
    });

    // ── Layout ────────────────────────────────────────────────────────────────
    let content = container(TabViewWrapper {
        id:    "tab-view-wrapper".to_string(),
        inner: tabs,
    }).padding(32.0);

    let root = rust_ui::row![nav, content].spacing(0.0);
    rust_ui_wgpu::run("rust-ui Showcase", 1100, 720, root, theme);
}
