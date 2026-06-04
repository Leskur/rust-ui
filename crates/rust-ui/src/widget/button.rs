//! Button widget.
//!
//! ```rust,ignore
//! use rust_ui::widget::button;
//!
//! let btn = button("Save")
//!     .variant(ButtonVariant::Primary)
//!     .on_click(|| println!("saved!"));
//! ```

use crate::color::Color;
use crate::event::{Event, EventStatus, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

/// Visual variant of the button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Filled with accent color (primary action).
    #[default]
    Primary,
    /// Subtle outline (secondary action).
    Secondary,
    /// Red / danger action.
    Danger,
    /// No background — text only.
    Ghost,
}

/// Interactive state tracked per-button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum State {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

pub struct Button {
    id:       String,
    label:    String,
    variant:  ButtonVariant,
    state:    State,
    disabled: bool,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id:       uuid(),
            label:    label.into(),
            variant:  ButtonVariant::Primary,
            state:    State::Normal,
            disabled: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self { self.variant = v; self }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self.state = if d { State::Disabled } else { State::Normal };
        self
    }
    pub fn on_click(mut self, f: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    fn colors(&self, theme: &Theme) -> (Color, Color) {
        let (bg, fg) = match self.variant {
            ButtonVariant::Primary   => (theme.accent,                    theme.accent_fg),
            ButtonVariant::Secondary => (theme.bg_elevated,               theme.fg),
            ButtonVariant::Danger    => (theme.danger,                    theme.danger_fg),
            ButtonVariant::Ghost     => (Color::TRANSPARENT,              theme.fg),
        };
        match self.state {
            State::Hovered  => (bg.lerp(Color::WHITE, 0.08), fg),
            State::Pressed  => (bg.lerp(Color::BLACK, 0.12), fg),
            State::Disabled => (bg.with_alpha(0.4),          fg.with_alpha(0.4)),
            State::Normal   => (bg, fg),
        }
    }
}

impl Widget for Button {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let (bg, fg) = self.colors(theme);
        let radius = Corners::all(theme.radius_md);

        renderer.fill_rect(bounds, bg, radius);

        if matches!(self.variant, ButtonVariant::Secondary) {
            renderer.stroke_rect(bounds, theme.border, 1.0, radius);
        }

        let opts = TextOptions {
            font_size: theme.font_size_md,
            color:     fg,
            ..Default::default()
        };
        let (tw, th) = renderer.measure_text(&self.label, &opts);
        let tx = bounds.x + (bounds.width  - tw) / 2.0;
        let ty = bounds.y + (bounds.height - th) / 2.0;
        renderer.draw_text(&self.label, Point::new(tx, ty), &opts);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled { return EventStatus::Ignored; }
        match event {
            Event::MouseMove { pos } => {
                let hovered = bounds.contains(pos.x, pos.y);
                self.state = if hovered {
                    if self.state == State::Pressed { State::Pressed } else { State::Hovered }
                } else {
                    State::Normal
                };
                EventStatus::Ignored
            }
            Event::MouseDown { pos, button: MouseButton::Left } => {
                if bounds.contains(pos.x, pos.y) {
                    self.state = State::Pressed;
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseUp { pos, button: MouseButton::Left } => {
                if self.state == State::Pressed && bounds.contains(pos.x, pos.y) {
                    self.state = State::Hovered;
                    if let Some(f) = &self.on_click { f(); }
                    EventStatus::Consumed
                } else {
                    self.state = State::Normal;
                    EventStatus::Ignored
                }
            }
            _ => EventStatus::Ignored,
        }
    }
}

/// Shorthand constructor.
pub fn button(label: impl Into<String>) -> Button {
    Button::new(label)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("btn-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
