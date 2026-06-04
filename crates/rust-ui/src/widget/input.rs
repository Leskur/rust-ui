//! Text input widget.

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

pub struct Input {
    id:          String,
    value:       String,
    placeholder: String,
    focused:     bool,
    hovered:     bool,
    on_change:   Option<Box<dyn Fn(&str)>>,
    on_submit:   Option<Box<dyn Fn(&str)>>,
}

impl Input {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            id:          uuid(),
            value:       value.into(),
            placeholder: String::new(),
            focused:     false,
            hovered:     false,
            on_change:   None,
            on_submit:   None,
        }
    }

    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into(); self
    }

    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(f)); self
    }

    pub fn on_submit(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_submit = Some(Box::new(f)); self
    }
}

impl Widget for Input {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let border_color = if self.focused {
            theme.accent
        } else if self.hovered {
            theme.fg_subtle
        } else {
            theme.border
        };

        let radius = Corners::all(theme.radius_md);
        renderer.fill_rect(bounds, theme.bg_elevated, radius);
        renderer.stroke_rect(bounds, border_color, 1.5, radius);

        let px = bounds.x + 10.0;
        let py = bounds.y + (bounds.height - theme.font_size_md) / 2.0;

        let (display_text, text_color) = if self.value.is_empty() {
            (self.placeholder.as_str(), theme.fg_subtle)
        } else {
            (self.value.as_str(), theme.fg)
        };

        let opts = TextOptions {
            font_size: theme.font_size_md,
            color: text_color,
            max_width: Some(bounds.width - 20.0),
            ..Default::default()
        };
        renderer.draw_text(display_text, Point::new(px, py), &opts);

        // Cursor
        if self.focused {
            let (tw, _) = renderer.measure_text(&self.value, &opts);
            let cx = px + tw + 1.0;
            renderer.draw_line(
                Point::new(cx, py),
                Point::new(cx, py + theme.font_size_md),
                theme.fg,
                1.5,
            );
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        match event {
            Event::MouseMove { pos } => {
                self.hovered = bounds.contains(pos.x, pos.y);
                EventStatus::Ignored
            }
            Event::MouseDown { pos, button: MouseButton::Left } => {
                self.focused = bounds.contains(pos.x, pos.y);
                EventStatus::Consumed
            }
            Event::TextInput { text } if self.focused => {
                self.value.push_str(text);
                if let Some(f) = &self.on_change { f(&self.value); }
                EventStatus::Consumed
            }
            Event::KeyDown { key, .. } if self.focused => {
                match key {
                    Key::Backspace => {
                        self.value.pop();
                        if let Some(f) = &self.on_change { f(&self.value); }
                    }
                    Key::Enter => {
                        if let Some(f) = &self.on_submit { f(&self.value); }
                    }
                    Key::Escape => { self.focused = false; }
                    _ => {}
                }
                EventStatus::Consumed
            }
            Event::FocusLost => {
                self.focused = false;
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }
}

/// Shorthand constructor.
pub fn input(value: impl Into<String>) -> Input {
    Input::new(value)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("input-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
