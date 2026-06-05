//! ScrollArea — scrollable container for content that exceeds bounds.
//!
//! Supports mouse wheel scrolling, content clipping, and scrollbar rendering.
//! Scrollbars appear only when content overflows (horizontal and vertical are
//! evaluated independently). Inspired by Radix UI ScrollArea design.
//!
//! ```rust,ignore
//! scroll_area(column![text("Item 1"), text("Item 2")])
//!     .max_width(600.0).max_height(400.0);
//! ```

use crate::color::Color;
use crate::event::{Event, EventStatus};
use crate::render::{Point, Rect, Renderer};
use crate::style::{Corners, Theme};
use crate::widget::Widget;
use std::cell::Cell;

const SCROLLBAR_W: f32 = 6.0; // scrollbar track width
const SCROLLBAR_PAD: f32 = 2.0; // padding from edge
const MIN_THUMB_SIZE: f32 = 24.0; // minimum thumb length

pub struct ScrollArea {
    id: String,
    child: Box<dyn Widget>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    scroll_x: f32,
    scroll_y: f32,
    hovered: bool,
    last_bounds: Cell<(f32, f32, f32, f32)>, // cached from draw() for correct event routing
    dragging_v: bool,
    dragging_h: bool,
    drag_start_pos: (f32, f32),
    drag_start_scroll: (f32, f32),
}

impl ScrollArea {
    pub fn new(child: impl Widget + 'static) -> Self {
        Self {
            id: uuid(),
            child: Box::new(child),
            max_width: None,
            max_height: None,
            scroll_x: 0.0,
            scroll_y: 0.0,
            hovered: false,
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            dragging_v: false,
            dragging_h: false,
            drag_start_pos: (0.0, 0.0),
            drag_start_scroll: (0.0, 0.0),
        }
    }

    /// Constrain the viewport to at most this width (default: fill parent).
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }

    /// Constrain the viewport to at most this height (default: fill parent).
    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = Some(h);
        self
    }

    /// Viewport always fits inside `bounds`; optionally capped by max_width/max_height.
    fn viewport(&self, bounds: Rect) -> Rect {
        let w = self
            .max_width
            .map_or(bounds.width, |mw| mw.min(bounds.width));
        let h = self
            .max_height
            .map_or(bounds.height, |mh| mh.min(bounds.height));
        Rect::new(bounds.x, bounds.y, w, h)
    }

    fn content_size(&self, theme: &Theme) -> (f32, f32) {
        self.child.intrinsic_size(theme)
    }

    fn max_scroll(&self, bounds: Rect, theme: &Theme) -> (f32, f32) {
        let vp = self.viewport(bounds);
        let (cw, ch) = self.content_size(theme);
        ((cw - vp.width).max(0.0), (ch - vp.height).max(0.0))
    }

    fn clamped_scroll(&self, bounds: Rect, theme: &Theme) -> (f32, f32) {
        let (mx, my) = self.max_scroll(bounds, theme);
        (self.scroll_x.clamp(0.0, mx), self.scroll_y.clamp(0.0, my))
    }

    /// Compute the vertical thumb rect for hit-testing and drawing.
    fn thumb_rect_v(&self, vp: &Rect, content_h: f32, scroll_y: f32) -> Option<Rect> {
        if content_h <= vp.height {
            return None;
        }
        let track_x = vp.x + vp.width - SCROLLBAR_W - SCROLLBAR_PAD;
        let track_y = vp.y + SCROLLBAR_PAD;
        let track_h = vp.height - SCROLLBAR_PAD * 2.0;
        let ratio = vp.height / content_h;
        let thumb_h = (track_h * ratio).max(MIN_THUMB_SIZE);
        let frac = scroll_y / (content_h - vp.height);
        let thumb_y = track_y + (track_h - thumb_h) * frac;
        Some(Rect::new(track_x, thumb_y, SCROLLBAR_W, thumb_h))
    }

    /// Compute the horizontal thumb rect for hit-testing and drawing.
    fn thumb_rect_h(&self, vp: &Rect, content_w: f32, scroll_x: f32) -> Option<Rect> {
        if content_w <= vp.width {
            return None;
        }
        let track_x = vp.x + SCROLLBAR_PAD;
        let track_y = vp.y + vp.height - SCROLLBAR_W - SCROLLBAR_PAD;
        let track_w = vp.width - SCROLLBAR_PAD * 2.0 - SCROLLBAR_W - SCROLLBAR_PAD;
        let ratio = vp.width / content_w;
        let thumb_w = (track_w * ratio).max(MIN_THUMB_SIZE);
        let frac = scroll_x / (content_w - vp.width);
        let thumb_x = track_x + (track_w - thumb_w) * frac;
        Some(Rect::new(thumb_x, track_y, thumb_w, SCROLLBAR_W))
    }

    fn draw_scrollbar_v(
        &self,
        renderer: &mut dyn Renderer,
        vp: &Rect,
        content_h: f32,
        scroll_y: f32,
        _theme: &Theme,
    ) {
        let Some(thumb) = self.thumb_rect_v(vp, content_h, scroll_y) else {
            return;
        };
        let track_x = vp.x + vp.width - SCROLLBAR_W - SCROLLBAR_PAD;
        let track_y = vp.y + SCROLLBAR_PAD;
        let track_h = vp.height - SCROLLBAR_PAD * 2.0;

        renderer.fill_rect(
            Rect::new(track_x, track_y, SCROLLBAR_W, track_h),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.05,
            },
            Corners::all(SCROLLBAR_W / 2.0),
        );
        renderer.fill_rect(
            thumb,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: if self.hovered { 0.35 } else { 0.2 },
            },
            Corners::all(SCROLLBAR_W / 2.0),
        );
    }

    fn draw_scrollbar_h(
        &self,
        renderer: &mut dyn Renderer,
        vp: &Rect,
        content_w: f32,
        scroll_x: f32,
        _theme: &Theme,
    ) {
        let Some(thumb) = self.thumb_rect_h(vp, content_w, scroll_x) else {
            return;
        };
        let track_x = vp.x + SCROLLBAR_PAD;
        let track_y = vp.y + vp.height - SCROLLBAR_W - SCROLLBAR_PAD;
        let track_w = vp.width - SCROLLBAR_PAD * 2.0 - SCROLLBAR_W - SCROLLBAR_PAD;

        renderer.fill_rect(
            Rect::new(track_x, track_y, track_w, SCROLLBAR_W),
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.05,
            },
            Corners::all(SCROLLBAR_W / 2.0),
        );
        renderer.fill_rect(
            thumb,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: if self.hovered { 0.35 } else { 0.2 },
            },
            Corners::all(SCROLLBAR_W / 2.0),
        );
    }
}

impl Widget for ScrollArea {
    fn id(&self) -> &str {
        &self.id
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        self.last_bounds
            .set((bounds.x, bounds.y, bounds.width, bounds.height));

        let vp = self.viewport(bounds);
        let (cw, ch) = self.content_size(theme);
        let (sx, sy) = self.clamped_scroll(bounds, theme);

        // Clip to viewport, then draw child offset by -scroll
        renderer.push_clip(vp);
        renderer.push_offset(-sx, -sy);
        let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
        self.child.draw(renderer, child_bounds, theme);
        renderer.pop_offset();

        // Scrollbars drawn inside clip (so they stay inside viewport)
        self.draw_scrollbar_v(renderer, &vp, ch, sy, theme);
        self.draw_scrollbar_h(renderer, &vp, cw, sx, theme);
        renderer.pop_clip();
    }

    fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let vp = self.viewport(bounds);
        let (cw, ch) = self.content_size(theme);
        let (sx, sy) = self.clamped_scroll(bounds, theme);
        renderer.push_offset(-sx, -sy);
        let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
        self.child.draw_overlay(renderer, child_bounds, theme);
        renderer.pop_offset();
    }

    fn handle_event(&mut self, event: &Event, _bounds: Rect) -> EventStatus {
        let (lx, ly, lw, lh) = self.last_bounds.get();
        let bounds = if lw > 0.0 {
            Rect::new(lx, ly, lw, lh)
        } else {
            _bounds
        };
        let vp = self.viewport(bounds);
        let theme_ref = &Theme::dark();
        let (mx, my) = self.max_scroll(bounds, theme_ref);
        let (cw, ch) = self.content_size(theme_ref);
        let (sx, sy) = self.clamped_scroll(bounds, theme_ref);

        // Pre-compute thumb rects for hit-testing
        let thumb_v = self.thumb_rect_v(&vp, ch, sy);
        let thumb_h = self.thumb_rect_h(&vp, cw, sx);

        match event {
            Event::MouseMove { pos } => {
                // Active drag overrides everything
                if self.dragging_v {
                    let track_h = vp.height - SCROLLBAR_PAD * 2.0;
                    let ratio = vp.height / ch;
                    let thumb_h = (track_h * ratio).max(MIN_THUMB_SIZE);
                    let dy = pos.y - self.drag_start_pos.1;
                    let delta = dy * (ch - vp.height) / (track_h - thumb_h).max(1.0);
                    self.scroll_y = (self.drag_start_scroll.1 + delta).clamp(0.0, my);
                    return EventStatus::Consumed;
                }
                if self.dragging_h {
                    let track_w = vp.width - SCROLLBAR_PAD * 2.0 - SCROLLBAR_W - SCROLLBAR_PAD;
                    let ratio = vp.width / cw;
                    let thumb_w = (track_w * ratio).max(MIN_THUMB_SIZE);
                    let dx = pos.x - self.drag_start_pos.0;
                    let delta = dx * (cw - vp.width) / (track_w - thumb_w).max(1.0);
                    self.scroll_x = (self.drag_start_scroll.0 + delta).clamp(0.0, mx);
                    return EventStatus::Consumed;
                }

                self.hovered = vp.contains(pos.x, pos.y);
                // Forward to child with scroll-adjusted coordinates
                if self.hovered {
                    let adj = Event::MouseMove {
                        pos: Point::new(pos.x + sx, pos.y + sy),
                    };
                    let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
                    self.child.handle_event(&adj, child_bounds);
                }
                EventStatus::Ignored
            }
            Event::Scroll {
                pos,
                delta_x,
                delta_y,
            } => {
                if !vp.contains(pos.x, pos.y) {
                    return EventStatus::Ignored;
                }
                let changed_x = mx > 0.0 && *delta_x != 0.0;
                let changed_y = my > 0.0 && *delta_y != 0.0;
                if changed_x || changed_y {
                    self.scroll_x = (self.scroll_x + delta_x).clamp(0.0, mx);
                    self.scroll_y = (self.scroll_y + delta_y).clamp(0.0, my);
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseDown { pos, button } => {
                // Check thumb hit before child forwarding
                if thumb_v.map_or(false, |r| r.contains(pos.x, pos.y)) {
                    self.dragging_v = true;
                    self.drag_start_pos = (pos.x, pos.y);
                    self.drag_start_scroll = (self.scroll_x, self.scroll_y);
                    return EventStatus::Consumed;
                }
                if thumb_h.map_or(false, |r| r.contains(pos.x, pos.y)) {
                    self.dragging_h = true;
                    self.drag_start_pos = (pos.x, pos.y);
                    self.drag_start_scroll = (self.scroll_x, self.scroll_y);
                    return EventStatus::Consumed;
                }

                if vp.contains(pos.x, pos.y) {
                    let adj = Event::MouseDown {
                        pos: Point::new(pos.x + sx, pos.y + sy),
                        button: *button,
                    };
                    let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
                    self.child.handle_event(&adj, child_bounds)
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseUp { pos, button } => {
                self.dragging_v = false;
                self.dragging_h = false;
                let adj = Event::MouseUp {
                    pos: Point::new(pos.x + sx, pos.y + sy),
                    button: *button,
                };
                let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
                self.child.handle_event(&adj, child_bounds)
            }
            // Keyboard / text / focus events — pass through directly
            Event::KeyDown { .. }
            | Event::KeyUp { .. }
            | Event::TextInput { .. }
            | Event::ImePreedit { .. }
            | Event::FocusGained
            | Event::FocusLost => {
                let child_bounds = Rect::new(vp.x, vp.y, cw.max(vp.width), ch.max(vp.height));
                self.child.handle_event(event, child_bounds)
            }
            _ => EventStatus::Ignored,
        }
    }

    fn layout_children<'a>(&'a self, _bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = Rect::new(bx, by, bw, bh);
        let vp = self.viewport(bounds);
        let (cw, ch) = self.child.intrinsic_size(theme);
        let child_bounds = Rect::new(
            vp.x - self.scroll_x,
            vp.y - self.scroll_y,
            cw.max(vp.width),
            ch.max(vp.height),
        );
        vec![(self.child.as_ref() as &dyn Widget, child_bounds)]
    }

    fn layout_children_mut<'a>(
        &'a mut self,
        _bounds: Rect,
        theme: &Theme,
    ) -> Vec<(&'a mut dyn Widget, Rect)> {
        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = Rect::new(bx, by, bw, bh);
        let vp = self.viewport(bounds);
        let (cw, ch) = self.child.intrinsic_size(theme);
        let child_bounds = Rect::new(
            vp.x - self.scroll_x,
            vp.y - self.scroll_y,
            cw.max(vp.width),
            ch.max(vp.height),
        );
        vec![(self.child.as_mut() as &mut dyn Widget, child_bounds)]
    }

    fn cursor_at(&self, pos: (f32, f32), _bounds: Rect) -> crate::style::CursorStyle {
        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = Rect::new(bx, by, bw, bh);
        let vp = self.viewport(bounds);
        if !vp.contains(pos.0, pos.1) {
            return crate::style::CursorStyle::Default;
        }
        let theme = Theme::default();
        for (child, cb) in self.layout_children(bounds, &theme) {
            return child.cursor_at(pos, cb);
        }
        crate::style::CursorStyle::Default
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        // When max dimensions are set, report those so parent Row/Column can
        // allocate correctly. Otherwise return 0 so the parent gives us
        // whatever space remains — actual bounds come from draw() via last_bounds.
        (
            self.max_width.unwrap_or(0.0),
            self.max_height.unwrap_or(0.0),
        )
    }
}

/// Shorthand constructor.
pub fn scroll_area(child: impl Widget + 'static) -> ScrollArea {
    ScrollArea::new(child)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("scroll-area-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
