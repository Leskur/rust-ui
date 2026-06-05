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
    container, scroll_area, sidebar, sidebar_group, sidebar_item, tab_view, Widget,
};
use rust_ui::widget::TabView;
use rust_ui::event::{Event, EventStatus};
use rust_ui::render::{Rect, Renderer};
use rust_ui::animation::AnimationScheduler;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let theme = Theme::dark();
    let scheduler = Arc::new(Mutex::new(AnimationScheduler::new()));

    // ── Content pages ─────────────────────────────────────────────────────────
    let tabs = Rc::new(RefCell::new(tab_view(vec![
        ("Overview".to_string(),  pages::overview::page()),
        ("Button".to_string(),    pages::button::page()),
        ("Input".to_string(),     pages::input::page()),
        ("Switch".to_string(),    pages::switch::page(scheduler.clone())),
        ("Container".to_string(), pages::container::page()),
        ("Layout".to_string(),    pages::layout::page()),
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
        fn layout_children<'a>(&'a self, bounds: Rect, theme: &rust_ui::style::Theme) -> Vec<(&'a dyn Widget, Rect)> {
            // TabViewWrapper can't easily return lifetime-safe refs from RefCell;
            // delegate cursor_at directly instead
            let _ = (bounds, theme);
            vec![]
        }
        fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> rust_ui::style::CursorStyle {
            self.inner.borrow().cursor_at(pos, bounds)
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
                sidebar_item("Layout"),
            ],
        ),
        sidebar_group(
            "Coming soon",
            vec![
                sidebar_item("Slider"),
                sidebar_item("Select"),
                sidebar_item("Checkbox"),
                sidebar_item("Dialog"),
                sidebar_item("Badge"),
                sidebar_item("Image"),
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

    let scrollable_content = scroll_area(content);

    // Custom root that splits bounds: sidebar on left, scroll area fills the rest.
    // This avoids Row's intrinsic_size-based allocation which would give ScrollArea
    // zero width when it reports intrinsic_size (0, 0).
    let sidebar_w = 220.0_f32;
    struct AppRoot {
        id:       String,
        sidebar:  Box<dyn Widget>,
        content:  Box<dyn Widget>,
        sidebar_w: f32,
    }
    impl Widget for AppRoot {
        fn id(&self) -> &str { &self.id }
        fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(bounds.x + self.sidebar_w, bounds.y, (bounds.width - self.sidebar_w).max(0.0), bounds.height);
            self.sidebar.draw(renderer, sb, theme);
            self.content.draw(renderer, cb, theme);
        }
        fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(bounds.x + self.sidebar_w, bounds.y, (bounds.width - self.sidebar_w).max(0.0), bounds.height);
            let s1 = self.sidebar.handle_event(event, sb);
            if s1 == EventStatus::Consumed { return EventStatus::Consumed; }
            self.content.handle_event(event, cb)
        }
        fn layout_children<'a>(&'a self, bounds: Rect, _theme: &rust_ui::style::Theme) -> Vec<(&'a dyn Widget, Rect)> {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(bounds.x + self.sidebar_w, bounds.y, (bounds.width - self.sidebar_w).max(0.0), bounds.height);
            vec![
                (self.sidebar.as_ref() as &dyn Widget, sb),
                (self.content.as_ref() as &dyn Widget, cb),
            ]
        }
    }

    let root = AppRoot {
        id:        "app-root".to_string(),
        sidebar:   Box::new(nav),
        content:   Box::new(scrollable_content),
        sidebar_w,
    };
    rust_ui_wgpu::run_with_scheduler("rust-ui Showcase", 1100, 720, root, theme, scheduler);
}
