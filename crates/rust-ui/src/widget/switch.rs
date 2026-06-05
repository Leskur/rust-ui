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
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchSize {
    Sm,
    Md,
    Lg,
}

impl SwitchSize {
    fn track_width(&self) -> f32 {
        match self {
            Self::Sm => 32.0,
            Self::Md => 40.0,
            Self::Lg => 48.0,
        }
    }
    fn track_height(&self) -> f32 {
        match self {
            Self::Sm => 18.0,
            Self::Md => 22.0,
            Self::Lg => 26.0,
        }
    }
    fn thumb_radius(&self) -> f32 {
        match self {
            Self::Sm => 6.0,
            Self::Md => 8.0,
            Self::Lg => 10.0,
        }
    }
}

const ANIM_DUR: f32 = 0.35; // seconds

pub struct Switch {
    id:         String,
    label:      String,
    on:         bool,
    hovered:    bool,
    disabled:   bool,
    size:       SwitchSize,
    on_change:  Option<Box<dyn Fn(bool)>>,
    scheduler:  Option<Arc<Mutex<AnimationScheduler>>>,
}

impl Switch {
    pub fn new(label: impl Into<String>, on: bool) -> Self {
        Self {
            id:         uuid(),
            label:      label.into(),
            on,
            hovered:    false,
            disabled:   false,
            size:       SwitchSize::Md,
            on_change:  None,
            scheduler:  None,
        }
    }

    pub fn on_change(mut self, f: impl Fn(bool) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn scheduler(mut self, scheduler: Arc<Mutex<AnimationScheduler>>) -> Self {
        self.scheduler = Some(scheduler);
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

        let track_w = self.size.track_width();
        let track_h = self.size.track_height();
        let thumb_r = self.size.thumb_radius();

        // Track background: lerp grey → accent
        let off_color = if self.disabled { theme.border } else { theme.bg_elevated };
        let on_color = if self.disabled { theme.border } else { theme.accent };
        let track_color = off_color.lerp(on_color, progress);

        let track_rect = Rect::new(bounds.x, bounds.y + (bounds.height - track_h) / 2.0, track_w, track_h);
        renderer.fill_rect(
            track_rect,
            track_color,
            crate::style::Corners::all(track_h / 2.0),
        );

        // Thumb position
        let x_off = bounds.x + track_h / 2.0;
        let x_on  = bounds.x + track_w - track_h / 2.0;
        let thumb_cx = x_off + (x_on - x_off) * progress;
        let thumb_cy = bounds.y + bounds.height / 2.0;

        // Shadow
        if !self.disabled {
            renderer.fill_circle(
                Point::new(thumb_cx, thumb_cy),
                thumb_r + 1.5,
                Color { r: 0.0, g: 0.0, b: 0.0, a: 0.15 },
            );
        }
        // Thumb
        let thumb_color = if self.disabled { theme.fg_muted } else { Color::WHITE };
        renderer.fill_circle(Point::new(thumb_cx, thumb_cy), thumb_r, thumb_color);

        // Label
        if !self.label.is_empty() {
            let text_x = bounds.x + track_w + 10.0;
            let label_color = if self.disabled { theme.fg_muted } else { if self.on { theme.fg } else { theme.fg_muted } };
            let opts = TextOptions {
                font_size: theme.font_size_md,
                color: label_color,
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
        // Use animation if scheduler is available, otherwise instant
        let progress = if let Some(scheduler) = &self.scheduler {
            if let Ok(sched) = scheduler.lock() {
                sched.get(&self.id, "progress").unwrap_or(if self.on { 1.0 } else { 0.0 })
            } else {
                if self.on { 1.0_f32 } else { 0.0 }
            }
        } else {
            if self.on { 1.0_f32 } else { 0.0 }
        };

        let track_w = self.size.track_width();
        let track_h = self.size.track_height();
        let thumb_r = self.size.thumb_radius();

        let off_color = if self.disabled { theme.border } else { theme.bg_elevated };
        let on_color = if self.disabled { theme.border } else { theme.accent };
        let track_color = off_color.lerp(on_color, progress);
        let track_rect = Rect::new(bounds.x, bounds.y + (bounds.height - track_h) / 2.0, track_w, track_h);
        renderer.fill_rect(track_rect, track_color, crate::style::Corners::all(track_h / 2.0));

        let x_off = bounds.x + track_h / 2.0;
        let x_on  = bounds.x + track_w - track_h / 2.0;
        let thumb_cx = x_off + (x_on - x_off) * progress;
        let thumb_cy = bounds.y + bounds.height / 2.0;

        // Shadow only when not disabled
        if !self.disabled {
            renderer.fill_circle(
                Point::new(thumb_cx, thumb_cy),
                thumb_r + 1.5,
                Color { r: 0.0, g: 0.0, b: 0.0, a: 0.15 },
            );
        }

        let thumb_color = if self.disabled { theme.fg_muted } else { Color::WHITE };
        renderer.fill_circle(Point::new(thumb_cx, thumb_cy), thumb_r, thumb_color);

        // Label
        if !self.label.is_empty() {
            let text_x = bounds.x + track_w + 10.0;
            let label_color = if self.disabled { theme.fg_muted } else { if self.on { theme.fg } else { theme.fg_muted } };
            let opts = TextOptions {
                font_size: theme.font_size_md,
                color: label_color,
                ..Default::default()
            };
            renderer.draw_text(
                &self.label,
                Point::new(text_x, bounds.y + (bounds.height - theme.font_size_md) / 2.0),
                &opts,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }
        match event {
            Event::MouseMove { pos } => {
                self.hovered = bounds.contains(pos.x, pos.y);
                EventStatus::Ignored
            }
            Event::MouseDown { pos, button: MouseButton::Left } => {
                if bounds.contains(pos.x, pos.y) {
                    self.on = !self.on;
                    if let Some(f) = &self.on_change { f(self.on); }
                    // Trigger animation
                    if let Some(scheduler) = &self.scheduler {
                        if let Ok(mut sched) = scheduler.lock() {
                            let target = if self.on { 1.0 } else { 0.0 };
                            sched.animate_to(&self.id, "progress", target, Easing::EaseOut, ANIM_DUR);
                        }
                    }
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            _ => EventStatus::Ignored,
        }
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if self.disabled {
            CursorStyle::Default
        } else if bounds.contains(pos.0, pos.1) {
            CursorStyle::Pointer
        } else {
            CursorStyle::Default
        }
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
