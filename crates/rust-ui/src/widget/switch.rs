//! Switch / Toggle widget with animated thumb.
//!
//! Animation is driven by the `AnimationScheduler` — no timer boilerplate needed.
//!
//! ```rust,ignore
//! let sw = switch("System Proxy", proxy_enabled)
//!     .on_change(|on| Message::SetProxy(on));
//! ```

use crate::animation::{AnimationScheduler, Easing};
use crate::color::Color;
use crate::event::{Event, EventStatus, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{CursorStyle, Theme};
use crate::widget::Widget;

const TRACK_W: f32 = 40.0;
const TRACK_H: f32 = 22.0;
const THUMB_R: f32 = 8.0;
const ANIM_DUR: f32 = 0.18; // seconds

pub struct Switch {
    id:        String,
    label:     String,
    on:        bool,
    hovered:   bool,
    on_change: Option<Box<dyn Fn(bool)>>,
}

impl Switch {
    pub fn new(label: impl Into<String>, on: bool) -> Self {
        Self {
            id:        uuid(),
            label:     label.into(),
            on,
            hovered:   false,
            on_change: None,
        }
    }

    pub fn on_change(mut self, f: impl Fn(bool) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Initialize animation state so the thumb starts at the correct position.
    pub fn init_anim(&self, scheduler: &mut AnimationScheduler) {
        let target = if self.on { 1.0 } else { 0.0 };
        scheduler.set(&self.id, "progress", target);
    }

    pub fn draw_with_anim(
        &self,
        renderer: &mut dyn Renderer,
        bounds: Rect,
        theme: &Theme,
        scheduler: &AnimationScheduler,
    ) {
        let progress = scheduler.get(&self.id, "progress").unwrap_or(if self.on { 1.0 } else { 0.0 });

        // Track background: lerp grey → accent
        let off_color = theme.bg_elevated;
        let track_color = off_color.lerp(theme.accent, progress);

        let track_rect = Rect::new(bounds.x, bounds.y + (bounds.height - TRACK_H) / 2.0, TRACK_W, TRACK_H);
        renderer.fill_rect(
            track_rect,
            track_color,
            crate::style::Corners::all(TRACK_H / 2.0),
        );

        // Thumb position
        let x_off = bounds.x + TRACK_H / 2.0;
        let x_on  = bounds.x + TRACK_W - TRACK_H / 2.0;
        let thumb_cx = x_off + (x_on - x_off) * progress;
        let thumb_cy = bounds.y + bounds.height / 2.0;

        // Shadow
        renderer.fill_circle(
            Point::new(thumb_cx, thumb_cy),
            THUMB_R + 1.5,
            Color { r: 0.0, g: 0.0, b: 0.0, a: 0.15 },
        );
        // Thumb
        renderer.fill_circle(Point::new(thumb_cx, thumb_cy), THUMB_R, Color::WHITE);

        // Label
        if !self.label.is_empty() {
            let text_x = bounds.x + TRACK_W + 10.0;
            let opts = TextOptions {
                font_size: theme.font_size_md,
                color: if self.on { theme.fg } else { theme.fg_muted },
                ..Default::default()
            };
            renderer.draw_text(
                &self.label,
                Point::new(text_x, bounds.y + (bounds.height - theme.font_size_md) / 2.0),
                &opts,
            );
        }
    }
}

impl Widget for Switch {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        // Fallback draw without scheduler (instant, no animation)
        let progress = if self.on { 1.0_f32 } else { 0.0 };
        let track_color = theme.bg_elevated.lerp(theme.accent, progress);
        let track_rect = Rect::new(bounds.x, bounds.y + (bounds.height - TRACK_H) / 2.0, TRACK_W, TRACK_H);
        renderer.fill_rect(track_rect, track_color, crate::style::Corners::all(TRACK_H / 2.0));

        let x_off = bounds.x + TRACK_H / 2.0;
        let x_on  = bounds.x + TRACK_W - TRACK_H / 2.0;
        let thumb_cx = x_off + (x_on - x_off) * progress;
        let thumb_cy = bounds.y + bounds.height / 2.0;
        renderer.fill_circle(Point::new(thumb_cx, thumb_cy), THUMB_R, Color::WHITE);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        match event {
            Event::MouseMove { pos } => {
                self.hovered = bounds.contains(pos.x, pos.y);
                EventStatus::Ignored
            }
            Event::MouseClick { pos, button: MouseButton::Left } => {
                if bounds.contains(pos.x, pos.y) {
                    self.on = !self.on;
                    if let Some(f) = &self.on_change { f(self.on); }
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            _ => EventStatus::Ignored,
        }
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if bounds.contains(pos.0, pos.1) { CursorStyle::Pointer } else { CursorStyle::Default }
    }
}

/// Shorthand constructor.
pub fn switch(label: impl Into<String>, on: bool) -> Switch {
    Switch::new(label, on)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("switch-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
