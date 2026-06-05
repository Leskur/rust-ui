//! rust-ui Showcase — interactive component browser.
//!
//! Left sidebar: component list
//! Right panel:  live demo of the selected component
//!
//! ```
//! cargo run -p showcase
//! ```

mod pages;

use rust_ui::animation::AnimationScheduler;
use rust_ui::event::{Event, EventStatus};
use rust_ui::prelude::*;
use rust_ui::render::{Rect, Renderer};
use rust_ui::widget::TabView;
use rust_ui::widget::{
    container, scroll_area, sidebar, sidebar_group, sidebar_item, tab_view, Widget,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let theme = Theme::dark();
    let scheduler = Arc::new(Mutex::new(AnimationScheduler::new()));
    let toast_store = toast_store();

    // ── Content pages ─────────────────────────────────────────────────────────
    let tabs = Rc::new(RefCell::new(
        tab_view(vec![
            ("Overview".to_string(), pages::overview::page()),
            ("Button".to_string(), pages::button::page()),
            ("Input".to_string(), pages::input::page()),
            ("Switch".to_string(), pages::switch::page(scheduler.clone())),
            ("Container".to_string(), pages::container::page()),
            ("Layout".to_string(), pages::layout::page()),
            ("Image".to_string(), pages::image::page()),
            ("Icon".to_string(), pages::icon::page()),
            ("Svg".to_string(), pages::svg::page()),
            ("Dialog".to_string(), pages::dialog::page()),
            ("Checkbox".to_string(), pages::checkbox::page()),
            ("Select".to_string(), pages::select::page()),
            ("Popover".to_string(), pages::popover::page()),
            ("Tooltip".to_string(), pages::tooltip::page()),
            ("Dropdown Menu".to_string(), pages::dropdown_menu::page()),
            ("Toast".to_string(), pages::toast::page(toast_store.clone())),
            ("Progress".to_string(), pages::progress::page()),
            ("Slider".to_string(), pages::slider::page()),
            ("Radio Group".to_string(), pages::radio_group::page()),
            ("Badge".to_string(), pages::badge::page()),
            ("Accordion".to_string(), pages::accordion::page()),
            ("Tabs".to_string(), pages::tabs::page()),
        ])
        .active("Button"),
    ));

    // Thin wrapper so Rc<RefCell<TabView>> can be used as a Widget
    struct TabViewWrapper {
        id: String,
        inner: Rc<RefCell<TabView>>,
    }
    impl Widget for TabViewWrapper {
        fn id(&self) -> &str {
            &self.id
        }
        fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            self.inner.borrow().draw(renderer, bounds, theme);
        }
        fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            self.inner.borrow().draw_overlay(renderer, bounds, theme);
        }
        fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
            self.inner.borrow_mut().handle_event(event, bounds)
        }
        fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
            self.inner.borrow().intrinsic_size(theme)
        }
        fn poll_redraw(&self, bounds: Rect, theme: &Theme) -> bool {
            self.inner.borrow().poll_redraw(bounds, theme)
        }
        fn layout_children<'a>(
            &'a self,
            bounds: Rect,
            theme: &rust_ui::style::Theme,
        ) -> Vec<(&'a dyn Widget, Rect)> {
            // TabViewWrapper can't return lifetime-safe refs from RefCell;
            // poll_redraw / cursor_at delegate to the inner TabView instead.
            let _ = (bounds, theme);
            vec![]
        }
        fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> rust_ui::style::CursorStyle {
            self.inner.borrow().cursor_at(pos, bounds)
        }
    }

    // ── Sidebar ───────────────────────────────────────────────────────────────
    let nav = scroll_area(
        sidebar(vec![
            sidebar_group("rust-ui", vec![sidebar_item("Overview")]),
            sidebar_group(
                "Components",
                vec![
                    sidebar_item("Button"),
                    sidebar_item("Input"),
                    sidebar_item("Switch"),
                    sidebar_item("Container"),
                    sidebar_item("Layout"),
                    sidebar_item("Image"),
                    sidebar_item("Icon"),
                    sidebar_item("Svg"),
                    sidebar_item("Dialog"),
                    sidebar_item("Checkbox"),
                    sidebar_item("Select"),
                    sidebar_item("Popover"),
                    sidebar_item("Tooltip"),
                    sidebar_item("Dropdown Menu"),
                    sidebar_item("Toast"),
                    sidebar_item("Progress"),
                    sidebar_item("Slider"),
                    sidebar_item("Radio Group"),
                    sidebar_item("Badge"),
                    sidebar_item("Accordion"),
                    sidebar_item("Tabs"),
                ],
            ),
        ])
        .width(220.0)
        .active("Button")
        .on_select({
            let tabs = tabs.clone();
            move |name: &str| {
                tabs.borrow_mut().set_active(name);
            }
        }),
    );

    // ── Layout ────────────────────────────────────────────────────────────────
    let content = container(TabViewWrapper {
        id: "tab-view-wrapper".to_string(),
        inner: tabs,
    })
    .padding(32.0);

    let scrollable_content = scroll_area(content);

    // Custom root that splits bounds: sidebar on left, scroll area fills the rest.
    // This avoids Row's intrinsic_size-based allocation which would give ScrollArea
    // zero width when it reports intrinsic_size (0, 0).
    let sidebar_w = 220.0_f32;
    struct AppRoot {
        id: String,
        sidebar: Box<dyn Widget>,
        content: Box<dyn Widget>,
        toaster: Box<dyn Widget>,
        sidebar_w: f32,
    }
    impl Widget for AppRoot {
        fn id(&self) -> &str {
            &self.id
        }
        fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            self.sidebar.draw(renderer, sb, theme);
            self.content.draw(renderer, cb, theme);
        }
        fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            self.sidebar.draw_overlay(renderer, sb, theme);
            self.content.draw_overlay(renderer, cb, theme);
            self.toaster.draw_overlay(renderer, bounds, theme);
        }
        fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            if self.toaster.handle_event(event, bounds) == EventStatus::Consumed {
                return EventStatus::Consumed;
            }

            // Route wheel scroll to the panel under the cursor.
            if let Event::Scroll { pos, .. } = event {
                if sb.contains(pos.x, pos.y) {
                    if self.sidebar.handle_event(event, sb) == EventStatus::Consumed {
                        return EventStatus::Consumed;
                    }
                }
                if cb.contains(pos.x, pos.y) {
                    return self.content.handle_event(event, cb);
                }
                return EventStatus::Ignored;
            }

            let s1 = self.sidebar.handle_event(event, sb);
            if s1 == EventStatus::Consumed {
                return EventStatus::Consumed;
            }
            self.content.handle_event(event, cb)
        }
        fn poll_redraw(&self, bounds: Rect, theme: &Theme) -> bool {
            if self.toaster.poll_redraw(bounds, theme) {
                return true;
            }
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            self.sidebar.poll_redraw(sb, theme) || self.content.poll_redraw(cb, theme)
        }
        fn layout_children<'a>(
            &'a self,
            bounds: Rect,
            _theme: &rust_ui::style::Theme,
        ) -> Vec<(&'a dyn Widget, Rect)> {
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            vec![
                (self.sidebar.as_ref() as &dyn Widget, sb),
                (self.content.as_ref() as &dyn Widget, cb),
                (self.toaster.as_ref() as &dyn Widget, bounds),
            ]
        }
        fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> rust_ui::style::CursorStyle {
            if let Some(c) = self.toaster_cursor(pos, bounds) {
                return c;
            }
            let sb = Rect::new(bounds.x, bounds.y, self.sidebar_w, bounds.height);
            let cb = Rect::new(
                bounds.x + self.sidebar_w,
                bounds.y,
                (bounds.width - self.sidebar_w).max(0.0),
                bounds.height,
            );
            if sb.contains(pos.0, pos.1) {
                return self.sidebar.cursor_at(pos, sb);
            }
            self.content.cursor_at(pos, cb)
        }
    }

    impl AppRoot {
        fn toaster_cursor(&self, pos: (f32, f32), bounds: Rect) -> Option<rust_ui::style::CursorStyle> {
            if self.toaster.hit_test(pos, bounds, &Theme::default()) {
                Some(self.toaster.cursor_at(pos, bounds))
            } else {
                None
            }
        }
    }

    let root = AppRoot {
        id: "app-root".to_string(),
        sidebar: Box::new(nav),
        content: Box::new(scrollable_content),
        toaster: Box::new(toaster(toast_store)),
        sidebar_w,
    };
    rust_ui_wgpu::run_with_scheduler("rust-ui Showcase", 1100, 720, root, theme, scheduler);
}
