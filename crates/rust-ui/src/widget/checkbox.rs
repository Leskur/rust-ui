//! Checkbox widget — boolean toggle with label.
//!
//! ```rust,ignore
//! checkbox("Remember me", true)
//!     .on_change(|v| println!("checked={v}"));
//! ```

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const BOX: f32 = 18.0;
const GAP: f32 = 10.0;

pub struct Checkbox {
    id: String,
    label: String,
    checked: bool,
    disabled: bool,
    hovered: bool,
    pressed: bool,
    focused: bool,
    on_change: Option<Box<dyn Fn(bool)>>,
}

impl Checkbox {
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self {
            id: uuid(),
            label: label.into(),
            checked,
            disabled: false,
            hovered: false,
            pressed: false,
            focused: false,
            on_change: None,
        }
    }

    pub fn checked(mut self, v: bool) -> Self {
        self.checked = v;
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn on_change(mut self, f: impl Fn(bool) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    fn toggle(&mut self) {
        self.checked = !self.checked;
        if let Some(f) = &self.on_change {
            f(self.checked);
        }
    }
}

impl Widget for Checkbox {
    fn id(&self) -> &str {
        &self.id
    }

    fn focusable(&self) -> bool {
        !self.disabled
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let bh = bounds.height.max(BOX);
        let box_y = bounds.y + (bh - BOX) / 2.0;
        let box_rect = Rect::new(bounds.x, box_y, BOX, BOX);

        let bg = if self.disabled {
            theme.bg_elevated.with_alpha(0.7)
        } else if self.checked {
            theme.accent
        } else {
            theme.bg_elevated
        };
        let border = if self.disabled {
            theme.border.with_alpha(0.5)
        } else if self.focused {
            theme.accent.with_alpha(0.9)
        } else if self.hovered {
            theme.fg_subtle
        } else {
            theme.border
        };

        renderer.fill_rect(box_rect, bg, Corners::all(5.0));
        renderer.stroke_rect(box_rect, border, 1.5, Corners::all(5.0));

        // Check mark
        if self.checked {
            let c = if self.disabled {
                theme.accent_fg.with_alpha(0.6)
            } else {
                theme.accent_fg
            };
            let p1 = Point::new(box_rect.x + 4.0, box_rect.y + 10.0);
            let p2 = Point::new(box_rect.x + 8.0, box_rect.y + 14.0);
            let p3 = Point::new(box_rect.x + 14.0, box_rect.y + 5.5);
            renderer.draw_line(p1, p2, c, 2.2);
            renderer.draw_line(p2, p3, c, 2.2);
        }

        // Focus ring (outer)
        if self.focused && !self.disabled {
            renderer.stroke_rect(
                Rect::new(
                    box_rect.x - 2.0,
                    box_rect.y - 2.0,
                    box_rect.width + 4.0,
                    box_rect.height + 4.0,
                ),
                theme.accent.with_alpha(0.55),
                2.0,
                Corners::all(7.0),
            );
        }

        // Label
        if !self.label.is_empty() {
            let color = if self.disabled { theme.fg_muted } else { theme.fg };
            let opts = TextOptions {
                font_size: theme.font_size_md,
                color,
                ..Default::default()
            };
            let tx = bounds.x + BOX + GAP;
            let (tw, th, _) = renderer.measure_text(&self.label, &opts);
            let ty = bounds.y + (bh - th) / 2.0;
            let _ = tw;
            renderer.draw_text(&self.label, Point::new(tx, ty), &opts);
        }

        // Press overlay
        if self.pressed && !self.disabled {
            renderer.fill_rect(box_rect, Color::BLACK.with_alpha(0.08), Corners::all(5.0));
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }

        match event {
            Event::FocusGained => {
                self.focused = true;
                EventStatus::Ignored
            }
            Event::FocusLost => {
                self.focused = false;
                self.pressed = false;
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Enter, .. } | Event::KeyDown { key: Key::Char(' '), .. } => {
                if self.focused {
                    self.toggle();
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                self.hovered = bounds.contains(pos.x, pos.y);
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if bounds.contains(pos.x, pos.y) {
                    self.pressed = true;
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseUp {
                pos,
                button: MouseButton::Left,
            } => {
                let inside = bounds.contains(pos.x, pos.y);
                let was_pressed = self.pressed;
                self.pressed = false;
                if was_pressed && inside {
                    self.toggle();
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let char_w = theme.font_size_md * 0.6;
        let label_w = self.label.chars().count() as f32 * char_w;
        let w = BOX + if self.label.is_empty() { 0.0 } else { GAP + label_w };
        (w.max(BOX), BOX.max(theme.font_size_md + 10.0))
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
pub fn checkbox(label: impl Into<String>, checked: bool) -> Checkbox {
    Checkbox::new(label, checked)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("checkbox-{}", CTR.fetch_add(1, Ordering::Relaxed))
}

