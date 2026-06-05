//! Tooltip — hover label anchored to a trigger (shadcn/ui inspired).
//!
//! ```rust,ignore
//! tooltip(button("Hover"), "Add to library")
//!     .side(PopoverSide::Top)
//!     .delay_ms(400);
//! ```

use std::cell::Cell;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::event::{Event, EventStatus};
use crate::overlay::{place_popup, viewport_around_anchor, PopoverAlign, PopoverSide};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const GAP: f32 = 6.0;
const PAD_X: f32 = 10.0;
const PAD_Y: f32 = 6.0;

pub struct Tooltip {
    id: String,
    trigger: Box<dyn Widget>,
    label: String,
    side: PopoverSide,
    align: PopoverAlign,
    delay_ms: u64,
    disabled: bool,
    hover_trigger: Cell<bool>,
    hover_panel: Cell<bool>,
    hover_start: Cell<Option<Instant>>,
    last_bounds: Cell<(f32, f32, f32, f32)>,
    last_trigger: Cell<(f32, f32, f32, f32)>,
    last_panel: Cell<(f32, f32, f32, f32)>,
}

impl Tooltip {
    pub fn new(trigger: impl Widget + 'static, label: impl Into<String>) -> Self {
        Self {
            id: uuid(),
            trigger: Box::new(trigger),
            label: label.into(),
            side: PopoverSide::Top,
            align: PopoverAlign::Center,
            delay_ms: 400,
            disabled: false,
            hover_trigger: Cell::new(false),
            hover_panel: Cell::new(false),
            hover_start: Cell::new(None),
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_trigger: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_panel: Cell::new((0.0, 0.0, 0.0, 0.0)),
        }
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
        self
    }

    pub fn delay_ms(mut self, ms: u64) -> Self {
        self.delay_ms = ms;
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    fn trigger_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let (tw, th) = self.trigger.intrinsic_size(theme);
        Rect::new(bounds.x, bounds.y, tw, th)
    }

    fn panel_size(&self, theme: &Theme) -> (f32, f32) {
        let char_w = theme.font_size_sm * 0.55;
        let w = self.label.chars().count() as f32 * char_w + PAD_X * 2.0;
        let h = theme.font_size_sm + PAD_Y * 2.0;
        (w.max(40.0), h.max(24.0))
    }

    fn panel_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let trigger = self.trigger_rect(bounds, theme);
        let (pw, ph) = self.panel_size(theme);
        let viewport = viewport_around_anchor(trigger, 32.0);
        place_popup(trigger, pw, ph, viewport, self.side, self.align, GAP)
    }

    fn is_hovering(&self) -> bool {
        self.hover_trigger.get() || self.hover_panel.get()
    }

    fn should_show(&self) -> bool {
        if self.disabled || !self.is_hovering() {
            return false;
        }
        if self.delay_ms == 0 {
            return true;
        }
        self.hover_start
            .get()
            .map(|t| t.elapsed() >= Duration::from_millis(self.delay_ms))
            .unwrap_or(false)
    }

    fn reset_hover(&self) {
        self.hover_trigger.set(false);
        self.hover_panel.set(false);
        self.hover_start.set(None);
    }

    fn panel_contains(&self, pos: Point) -> bool {
        let (x, y, w, h) = self.last_panel.get();
        if w <= 0.0 {
            return false;
        }
        Rect::new(x, y, w, h).contains(pos.x, pos.y)
    }

    fn update_hover(&self, pos: Point, bounds: Rect, theme: &Theme) {
        if self.disabled {
            self.reset_hover();
            return;
        }

        let trigger = self.trigger_rect(bounds, theme);
        let on_trigger = trigger.contains(pos.x, pos.y);
        let on_panel = self.should_show() && self.panel_contains(pos);

        if on_trigger {
            self.hover_trigger.set(true);
            if self.hover_start.get().is_none() {
                self.hover_start.set(Some(Instant::now()));
            }
        } else if on_panel {
            self.hover_panel.set(true);
            self.hover_trigger.set(true);
        } else {
            self.reset_hover();
        }
    }
}

impl Widget for Tooltip {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn wants_redraw(&self) -> bool {
        if self.disabled || self.should_show() {
            return false;
        }
        if !self.hover_trigger.get() {
            return false;
        }
        if self.delay_ms == 0 {
            return false;
        }
        self.hover_start
            .get()
            .map(|t| t.elapsed() < Duration::from_millis(self.delay_ms))
            .unwrap_or(false)
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
        if !self.should_show() {
            self.last_panel.set((0.0, 0.0, 0.0, 0.0));
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

        // shadcn tooltip: dark filled pill
        let bg = theme.fg;
        renderer.fill_rect(panel, bg, Corners::all(theme.radius_sm));
        renderer.stroke_rect(
            panel,
            Color::WHITE.with_alpha(0.08),
            1.0,
            Corners::all(theme.radius_sm),
        );

        let opts = TextOptions {
            font_size: theme.font_size_sm,
            color: theme.bg,
            ..Default::default()
        };
        let (_, th, _) = renderer.measure_text(&self.label, &opts);
        renderer.draw_text(
            &self.label,
            Point::new(
                panel.x + PAD_X,
                panel.y + (panel.height - th) / 2.0,
            ),
            &opts,
        );
    }

    fn hit_test(&self, pos: (f32, f32), bounds: Rect, theme: &Theme) -> bool {
        let trigger = self.trigger_rect(bounds, theme);
        if trigger.contains(pos.0, pos.1) {
            return true;
        }
        if self.should_show() {
            let (x, y, w, h) = self.last_panel.get();
            if w > 0.0 && Rect::new(x, y, w, h).contains(pos.0, pos.1) {
                return true;
            }
        }
        false
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let trigger = self.trigger_rect(bounds, &theme);

        match event {
            Event::MouseMove { pos } => {
                self.update_hover(*pos, bounds, &theme);
                if self.should_show() && self.panel_contains(*pos) {
                    return EventStatus::Ignored;
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::MouseDown { pos, .. } | Event::MouseUp { pos, .. } => {
                if self.should_show() && self.panel_contains(*pos) {
                    return EventStatus::Consumed;
                }
                self.trigger.handle_event(event, trigger)
            }
            _ => self.trigger.handle_event(event, trigger),
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        self.trigger.intrinsic_size(theme)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        let trigger = self.trigger_rect(bounds, &theme);
        self.trigger.cursor_at(pos, trigger)
    }
}

/// Shorthand constructor.
pub fn tooltip(trigger: impl Widget + 'static, label: impl Into<String>) -> Tooltip {
    Tooltip::new(trigger, label)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("tooltip-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
