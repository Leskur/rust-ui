//! Popover — floating panel anchored to a trigger (shadcn/ui inspired).
//!
//! ```rust,ignore
//! let open = Rc::new(RefCell::new(false));
//!
//! popover(
//!     button("Open popover").on_click({ let o = open.clone(); move || *o.borrow_mut() = true }),
//!     column![
//!         text("Dimensions").size(14.0).bold(),
//!         text("Set the dimensions for the layer.").size(12.0),
//!     ],
//! )
//! .shared_open(open)
//! .width(320.0);
//! ```

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::overlay::{place_popup, viewport_around_anchor, PopoverAlign, PopoverSide};
use crate::render::{Point, Rect, Renderer};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const GAP: f32 = 8.0;
const CONTENT_PAD: f32 = 16.0;

pub struct Popover {
    id: String,
    trigger: Box<dyn Widget>,
    content: Box<dyn Widget>,
    open: Rc<RefCell<bool>>,
    side: PopoverSide,
    align: PopoverAlign,
    content_width: Option<f32>,
    on_close: Option<Box<dyn Fn()>>,
    last_bounds: Cell<(f32, f32, f32, f32)>,
    last_trigger: Cell<(f32, f32, f32, f32)>,
    last_panel: Cell<(f32, f32, f32, f32)>,
}

impl Popover {
    pub fn new(trigger: impl Widget + 'static, content: impl Widget + 'static) -> Self {
        Self {
            id: uuid(),
            trigger: Box::new(trigger),
            content: Box::new(content),
            open: Rc::new(RefCell::new(false)),
            side: PopoverSide::Bottom,
            align: PopoverAlign::Start,
            content_width: None,
            on_close: None,
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_trigger: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_panel: Cell::new((0.0, 0.0, 0.0, 0.0)),
        }
    }

    pub fn shared_open(mut self, flag: Rc<RefCell<bool>>) -> Self {
        self.open = flag;
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.content_width = Some(w);
        self
    }

    pub fn on_close(mut self, f: impl Fn() + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }

    pub fn is_open(&self) -> bool {
        *self.open.borrow()
    }

    fn close(&self) {
        *self.open.borrow_mut() = false;
        if let Some(f) = &self.on_close {
            f();
        }
    }

    fn trigger_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let (tw, th) = self.trigger.intrinsic_size(theme);
        Rect::new(bounds.x, bounds.y, tw, th)
    }

    fn panel_size(&self, theme: &Theme) -> (f32, f32) {
        let (cw, ch) = self.content.intrinsic_size(theme);
        let w = self.content_width.unwrap_or((cw + CONTENT_PAD * 2.0).max(240.0));
        let h = ch + CONTENT_PAD * 2.0;
        (w, h.max(80.0))
    }

    fn panel_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let trigger = self.trigger_rect(bounds, theme);
        let (pw, ph) = self.panel_size(theme);
        let viewport = viewport_around_anchor(trigger, 24.0);
        place_popup(trigger, pw, ph, viewport, self.side, self.align, GAP)
    }

    fn content_rect(&self, panel: Rect) -> Rect {
        Rect::new(
            panel.x + CONTENT_PAD,
            panel.y + CONTENT_PAD,
            panel.width - CONTENT_PAD * 2.0,
            panel.height - CONTENT_PAD * 2.0,
        )
    }

    fn panel_contains(&self, pos: Point) -> bool {
        let (x, y, w, h) = self.last_panel.get();
        Rect::new(x, y, w, h).contains(pos.x, pos.y)
    }

    fn trigger_contains(&self, pos: Point) -> bool {
        let (x, y, w, h) = self.last_trigger.get();
        Rect::new(x, y, w, h).contains(pos.x, pos.y)
    }
}

impl Widget for Popover {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        self.last_bounds
            .set((bounds.x, bounds.y, bounds.width, bounds.height));

        let trigger = self.trigger_rect(bounds, theme);
        self.last_trigger
            .set((trigger.x, trigger.y, trigger.width, trigger.height));
        self.trigger.draw(renderer, trigger, theme);
    }

    fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        if !self.is_open() {
            return;
        }

        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = if bw > 0.0 {
            Rect::new(bx, by, bw, bh)
        } else {
            bounds
        };

        let panel = self.panel_rect(bounds, theme);
        self.last_panel
            .set((panel.x, panel.y, panel.width, panel.height));

        // Shadow
        renderer.fill_rect(
            Rect::new(panel.x + 1.0, panel.y + 2.0, panel.width, panel.height),
            Color::BLACK.with_alpha(0.2),
            Corners::all(theme.radius_lg),
        );

        renderer.fill_rect(panel, theme.bg_surface, Corners::all(theme.radius_lg));
        renderer.stroke_rect(panel, theme.border, 1.0, Corners::all(theme.radius_lg));

        let inner = self.content_rect(panel);
        self.content.draw(renderer, inner, theme);
    }

    fn hit_test(&self, pos: (f32, f32), bounds: Rect, theme: &Theme) -> bool {
        let trigger = self.trigger_rect(bounds, theme);
        if trigger.contains(pos.0, pos.1) {
            return true;
        }
        if self.is_open() && self.panel_contains(Point::new(pos.0, pos.1)) {
            return true;
        }
        false
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let trigger = self.trigger_rect(bounds, &theme);
        let panel = self.panel_rect(bounds, &theme);
        let content_bounds = self.content_rect(panel);

        match event {
            Event::KeyDown { key: Key::Escape, .. } if self.is_open() => {
                self.close();
                EventStatus::Consumed
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if self.is_open() {
                    if self.panel_contains(*pos) {
                        return self.content.handle_event(event, content_bounds);
                    }
                    if !self.trigger_contains(*pos) {
                        self.close();
                        return EventStatus::Consumed;
                    }
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::MouseUp {
                pos,
                button: MouseButton::Left,
            } => {
                if self.is_open() && self.panel_contains(*pos) {
                    return self.content.handle_event(event, content_bounds);
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::MouseMove { pos } => {
                if self.is_open() && self.panel_contains(*pos) {
                    return self.content.handle_event(event, content_bounds);
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::Scroll { pos, .. } => {
                if self.is_open()
                    && (self.panel_contains(*pos) || self.trigger_contains(*pos))
                {
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            _ if self.is_open() => self.content.handle_event(event, content_bounds),
            _ => self.trigger.handle_event(event, trigger),
        }
    }

    fn layout_children<'a>(&'a self, bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        if !self.is_open() {
            return vec![];
        }
        let panel = self.panel_rect(bounds, theme);
        vec![(
            self.content.as_ref() as &dyn Widget,
            self.content_rect(panel),
        )]
    }

    fn layout_children_mut<'a>(
        &'a mut self,
        bounds: Rect,
        theme: &Theme,
    ) -> Vec<(&'a mut dyn Widget, Rect)> {
        if !self.is_open() {
            return vec![];
        }
        let panel = self.panel_rect(bounds, theme);
        let rect = self.content_rect(panel);
        vec![(self.content.as_mut() as &mut dyn Widget, rect)]
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        self.trigger.intrinsic_size(theme)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        if self.is_open() {
            let panel = self.panel_rect(bounds, &theme);
            let inner = self.content_rect(panel);
            if inner.contains(pos.0, pos.1) {
                return self.content.cursor_at(pos, inner);
            }
        }
        let trigger = self.trigger_rect(bounds, &theme);
        self.trigger.cursor_at(pos, trigger)
    }
}

/// Shorthand constructor.
pub fn popover(trigger: impl Widget + 'static, content: impl Widget + 'static) -> Popover {
    Popover::new(trigger, content)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("popover-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
