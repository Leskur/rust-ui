//! Slider — draggable value control (shadcn/ui inspired).
//!
//! ```rust,ignore
//! slider(50.0)
//!     .range(0.0, 100.0)
//!     .step(1.0)
//!     .width(280.0)
//!     .on_change(|v| println!("{v}"));
//! ```

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const DEFAULT_WIDTH: f32 = 240.0;
const TRACK_H: f32 = 6.0;
const THUMB_R: f32 = 8.0;
const VALUE_LABEL_W: f32 = 36.0;

pub struct Slider {
    id: String,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    width: Option<f32>,
    disabled: bool,
    focused: bool,
    dragging: bool,
    show_value: bool,
    on_change: Option<Box<dyn Fn(f32)>>,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self {
            id: uuid(),
            value: value.clamp(0.0, 100.0),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            width: None,
            disabled: false,
            focused: false,
            dragging: false,
            show_value: false,
            on_change: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max.max(min + 0.001);
        self.value = self.value.clamp(self.min, self.max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step.max(0.001);
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn show_value(mut self, v: bool) -> Self {
        self.show_value = v;
        self
    }

    pub fn on_change(mut self, f: impl Fn(f32) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    fn track_width(&self, bounds: Rect) -> f32 {
        let base = self.width.unwrap_or(DEFAULT_WIDTH);
        let label = if self.show_value { VALUE_LABEL_W + 8.0 } else { 0.0 };
        (base - label).min(bounds.width.max(DEFAULT_WIDTH) - label)
    }

    fn track_rect(&self, bounds: Rect) -> Rect {
        let w = self.track_width(bounds);
        Rect::new(
            bounds.x,
            bounds.y + (bounds.height - TRACK_H) / 2.0,
            w,
            TRACK_H,
        )
    }

    fn fraction(&self) -> f32 {
        ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
    }

    fn thumb_center(&self, track: Rect) -> Point {
        let x = track.x + track.width * self.fraction();
        Point::new(x, track.y + track.height / 2.0)
    }

    fn snap(&self, raw: f32) -> f32 {
        if self.step <= 0.0 {
            return raw.clamp(self.min, self.max);
        }
        let steps = ((raw - self.min) / self.step).round();
        (self.min + steps * self.step).clamp(self.min, self.max)
    }

    fn set_value(&mut self, raw: f32) {
        let next = self.snap(raw);
        if (next - self.value).abs() < 0.0001 {
            return;
        }
        self.value = next;
        if let Some(f) = &self.on_change {
            f(self.value);
        }
    }

    fn value_from_x(&self, x: f32, track: Rect) -> f32 {
        let t = ((x - track.x) / track.width.max(0.001)).clamp(0.0, 1.0);
        self.min + t * (self.max - self.min)
    }

    fn nudge(&mut self, delta_steps: f32) {
        self.set_value(self.value + delta_steps * self.step);
    }

    fn hit_thumb(&self, pos: Point, track: Rect) -> bool {
        let c = self.thumb_center(track);
        let dx = pos.x - c.x;
        let dy = pos.y - c.y;
        dx * dx + dy * dy <= (THUMB_R + 4.0).powi(2)
    }
}

impl Widget for Slider {
    fn id(&self) -> &str {
        &self.id
    }

    fn focusable(&self) -> bool {
        !self.disabled
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let track = self.track_rect(bounds);
        let radius = Corners::all(track.height / 2.0);

        renderer.fill_rect(track, theme.bg_elevated, radius);
        renderer.stroke_rect(track, theme.border.with_alpha(0.5), 1.0, radius);

        let fill_w = track.width * self.fraction();
        if fill_w > 0.0 {
            renderer.fill_rect(
                Rect::new(track.x, track.y, fill_w.max(track.height), track.height),
                if self.disabled {
                    theme.fg_muted
                } else {
                    theme.accent
                },
                radius,
            );
        }

        let thumb = self.thumb_center(track);
        if !self.disabled {
            renderer.fill_circle(
                thumb,
                THUMB_R + 1.5,
                Color::BLACK.with_alpha(0.12),
            );
        }
        renderer.fill_circle(
            thumb,
            if self.dragging && !self.disabled {
                THUMB_R - 1.0
            } else {
                THUMB_R
            },
            if self.disabled {
                theme.fg_muted
            } else {
                Color::WHITE
            },
        );

        if self.focused && !self.disabled {
            renderer.stroke_rect(
                Rect::new(
                    track.x - 2.0,
                    bounds.y,
                    track.width + 4.0,
                    bounds.height,
                ),
                theme.accent.with_alpha(0.45),
                2.0,
                Corners::all(6.0),
            );
        }

        if self.show_value {
            let label = format_value(self.value);
            let opts = TextOptions {
                font_size: theme.font_size_sm,
                color: if self.disabled {
                    theme.fg_muted
                } else {
                    theme.fg
                },
                ..Default::default()
            };
            renderer.draw_text(
                &label,
                Point::new(
                    track.x + track.width + 8.0,
                    bounds.y + (bounds.height - theme.font_size_sm) / 2.0,
                ),
                &opts,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }
        let track = self.track_rect(bounds);

        match event {
            Event::FocusGained => {
                self.focused = true;
                EventStatus::Ignored
            }
            Event::FocusLost => {
                self.focused = false;
                self.dragging = false;
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowRight, .. } | Event::KeyDown { key: Key::ArrowUp, .. } => {
                if self.focused {
                    self.nudge(1.0);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowLeft, .. } | Event::KeyDown { key: Key::ArrowDown, .. } => {
                if self.focused {
                    self.nudge(-1.0);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Home, .. } => {
                if self.focused {
                    self.set_value(self.min);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::End, .. } => {
                if self.focused {
                    self.set_value(self.max);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                if self.dragging {
                    self.set_value(self.value_from_x(pos.x, track));
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if track.contains(pos.x, pos.y) || self.hit_thumb(*pos, track) {
                    self.dragging = true;
                    self.set_value(self.value_from_x(pos.x, track));
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseUp {
                button: MouseButton::Left,
                ..
            } => {
                if self.dragging {
                    self.dragging = false;
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        let w = self.width.unwrap_or(DEFAULT_WIDTH);
        (w, (THUMB_R * 2.0 + 8.0).max(24.0))
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if self.disabled {
            return CursorStyle::Default;
        }
        let track = self.track_rect(bounds);
        let p = Point::new(pos.0, pos.1);
        if self.hit_thumb(p, track) || track.contains(p.x, p.y) {
            CursorStyle::Pointer
        } else {
            CursorStyle::Default
        }
    }
}

fn format_value(v: f32) -> String {
    if (v - v.round()).abs() < 0.01 {
        format!("{:.0}", v)
    } else {
        format!("{v:.1}")
    }
}

/// Shorthand constructor.
pub fn slider(value: f32) -> Slider {
    Slider::new(value)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("slider-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
