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
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

/// Visual variant of the button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Filled with accent color (primary action).
    #[default]
    Primary,
    /// Subtle background (secondary action).
    Secondary,
    /// Transparent background with visible border.
    Outline,
    /// Red / danger action.
    Danger,
    /// No background, no border — text only.
    Ghost,
}

/// Size scale for the button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
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
    id: String,
    label: String,
    variant: ButtonVariant,
    size: ButtonSize,
    state: State,
    focused: bool,
    disabled: bool,
    loading: bool,
    full_width: bool,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: uuid(),
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            state: State::Normal,
            focused: false,
            disabled: false,
            loading: false,
            full_width: false,
            on_click: None,
        }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        if l {
            self.state = State::Disabled;
        }
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self.state = if d { State::Disabled } else { State::Normal };
        self
    }
    pub fn on_click(mut self, f: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }

    /// (h_padding, v_padding, font_size) for each size
    fn size_metrics(&self, theme: &Theme) -> (f32, f32, f32) {
        match self.size {
            ButtonSize::Xs => (8.0, 3.0, theme.font_size_sm),
            ButtonSize::Sm => (12.0, 5.0, theme.font_size_sm),
            ButtonSize::Md => (16.0, 8.0, theme.font_size_md),
            ButtonSize::Lg => (22.0, 11.0, theme.font_size_lg),
        }
    }

    fn is_interactive(&self) -> bool {
        !self.disabled && !self.loading
    }

    fn colors(&self, theme: &Theme) -> (Color, Color) {
        let (bg, fg) = match self.variant {
            ButtonVariant::Primary => (theme.accent, theme.accent_fg),
            ButtonVariant::Secondary => (theme.bg_elevated, theme.fg),
            ButtonVariant::Outline => (Color::TRANSPARENT, theme.fg),
            ButtonVariant::Danger => (theme.danger, theme.danger_fg),
            ButtonVariant::Ghost => (Color::TRANSPARENT, theme.fg),
        };
        match self.state {
            State::Hovered => (bg.lerp(Color::WHITE, 0.08), fg),
            State::Pressed => (bg.lerp(Color::BLACK, 0.12), fg),
            State::Disabled => (bg.with_alpha(0.4), fg.with_alpha(0.4)),
            State::Normal => (bg, fg),
        }
    }
}

impl Widget for Button {
    fn id(&self) -> &str {
        &self.id
    }

    fn focusable(&self) -> bool {
        self.state != State::Disabled
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let (bg, fg) = self.colors(theme);
        let (_, _, font_size) = self.size_metrics(theme);
        let radius = Corners::all(theme.radius_md);

        if bg.a > 0.0 {
            renderer.fill_rect(bounds, bg, radius);
        }

        // Border for Secondary and Outline
        let needs_border = matches!(
            self.variant,
            ButtonVariant::Secondary | ButtonVariant::Outline
        );
        if needs_border {
            let border_color = if self.state == State::Disabled {
                theme.border.with_alpha(0.4)
            } else {
                theme.border
            };
            renderer.stroke_rect(bounds, border_color, 1.0, radius);
        }

        // Focus ring
        if self.focused && self.is_interactive() {
            renderer.stroke_rect(
                Rect::new(bounds.x - 2.0, bounds.y - 2.0, bounds.width + 4.0, bounds.height + 4.0),
                theme.accent.with_alpha(0.85),
                2.0,
                Corners::all(theme.radius_md + 2.0),
            );
        }

        // Label — show "…" when loading
        let display_label = if self.loading { "…" } else { &self.label };

        let opts = TextOptions {
            font_size,
            color: fg,
            ..Default::default()
        };
        let (tw, th, _ascent) = renderer.measure_text(display_label, &opts);
        // Standard vertical center: place text bounding box in the middle of the button
        let tx = bounds.x + (bounds.width - tw) / 2.0;
        let ty = bounds.y + (bounds.height - th) / 2.0;
        renderer.draw_text(display_label, Point::new(tx, ty), &opts);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        match event {
            Event::FocusGained => {
                self.focused = true;
                EventStatus::Ignored
            }
            Event::FocusLost => {
                self.focused = false;
                self.state = if self.disabled || self.loading {
                    State::Disabled
                } else {
                    State::Normal
                };
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Enter, .. } | Event::KeyDown { key: Key::Char(' '), .. } => {
                if self.focused && self.is_interactive() {
                    if let Some(f) = &self.on_click {
                        f();
                    }
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                if !self.is_interactive() {
                    return EventStatus::Ignored;
                }
                let hovered = bounds.contains(pos.x, pos.y);
                self.state = if hovered {
                    if self.state == State::Pressed {
                        State::Pressed
                    } else {
                        State::Hovered
                    }
                } else {
                    State::Normal
                };
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if !self.is_interactive() {
                    return EventStatus::Ignored;
                }
                if bounds.contains(pos.x, pos.y) {
                    self.state = State::Pressed;
                    EventStatus::Consumed
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseUp {
                pos,
                button: MouseButton::Left,
            } => {
                if !self.is_interactive() {
                    return EventStatus::Ignored;
                }
                if self.state == State::Pressed && bounds.contains(pos.x, pos.y) {
                    self.state = State::Hovered;
                    if let Some(f) = &self.on_click {
                        f();
                    }
                    EventStatus::Consumed
                } else {
                    self.state = State::Normal;
                    EventStatus::Ignored
                }
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let (hp, vp, fs) = self.size_metrics(theme);
        let char_w = fs * 0.6;
        let text_w = self.label.chars().count() as f32 * char_w;
        let w = (text_w + hp * 2.0).max(60.0);
        let h = fs + vp * 2.0;
        (w, h)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if !bounds.contains(pos.0, pos.1) {
            return CursorStyle::Default;
        }
        if self.disabled || self.loading {
            CursorStyle::NotAllowed
        } else {
            CursorStyle::Pointer
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
