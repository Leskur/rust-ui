//! Radio Group — mutually exclusive options (shadcn/ui inspired).
//!
//! ```rust,ignore
//! let selected = Rc::new(RefCell::new("comfortable".to_string()));
//!
//! radio_group(
//!     vec![
//!         ("default", "Default"),
//!         ("comfortable", "Comfortable"),
//!         ("compact", "Compact"),
//!     ],
//!     selected.clone(),
//! )
//! .on_change(|value| println!("{value}"));
//! ```

use std::cell::RefCell;
use std::rc::Rc;

use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const CIRCLE: f32 = 16.0;
const INNER: f32 = 8.0;
const GAP: f32 = 10.0;
const ROW_H: f32 = 24.0;
const DEFAULT_SPACING: f32 = 10.0;

/// One option inside a [`RadioGroup`].
#[derive(Debug, Clone)]
pub struct RadioGroupOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl RadioGroupOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
}

pub struct RadioGroup {
    id: String,
    options: Vec<RadioGroupOption>,
    selected: Rc<RefCell<String>>,
    spacing: f32,
    disabled: bool,
    hovered: Option<usize>,
    focused: Option<usize>,
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl RadioGroup {
    pub fn new(options: Vec<RadioGroupOption>, selected: Rc<RefCell<String>>) -> Self {
        if let Some(first) = options.first() {
            if selected.borrow().is_empty() {
                *selected.borrow_mut() = first.value.clone();
            }
        }
        Self {
            id: uuid(),
            options,
            selected,
            spacing: DEFAULT_SPACING,
            disabled: false,
            hovered: None,
            focused: None,
            on_change: None,
        }
    }

    pub fn from_pairs(
        options: Vec<(impl Into<String>, impl Into<String>)>,
        selected: Rc<RefCell<String>>,
    ) -> Self {
        Self::new(
            options
                .into_iter()
                .map(|(v, l)| RadioGroupOption::new(v, l))
                .collect(),
            selected,
        )
    }

    pub fn spacing(mut self, gap: f32) -> Self {
        self.spacing = gap.max(0.0);
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }

    pub fn disabled_option(mut self, value: impl Into<String>) -> Self {
        let value = value.into();
        if let Some(opt) = self.options.iter_mut().find(|o| o.value == value) {
            opt.disabled = true;
        }
        self
    }

    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    fn row_rect(&self, index: usize, bounds: Rect) -> Rect {
        let y = bounds.y + index as f32 * (ROW_H + self.spacing);
        Rect::new(bounds.x, y, bounds.width, ROW_H)
    }

    fn index_at(&self, pos: Point, bounds: Rect) -> Option<usize> {
        for i in 0..self.options.len() {
            if self.row_rect(i, bounds).contains(pos.x, pos.y) {
                return Some(i);
            }
        }
        None
    }

    fn is_selected(&self, index: usize) -> bool {
        self.options
            .get(index)
            .map(|o| *self.selected.borrow() == o.value)
            .unwrap_or(false)
    }

    fn selectable_indices(&self) -> Vec<usize> {
        self.options
            .iter()
            .enumerate()
            .filter_map(|(i, o)| {
                if o.disabled || self.disabled {
                    None
                } else {
                    Some(i)
                }
            })
            .collect()
    }

    fn select_index(&mut self, index: usize) {
        let Some(opt) = self.options.get(index) else {
            return;
        };
        if opt.disabled || self.disabled {
            return;
        }
        let value = opt.value.clone();
        if *self.selected.borrow() == value {
            return;
        }
        *self.selected.borrow_mut() = value.clone();
        if let Some(f) = &self.on_change {
            f(&value);
        }
    }

    fn focus_selected_index(&mut self) {
        let current = self.selected.borrow().clone();
        self.focused = self
            .options
            .iter()
            .position(|o| o.value == current && !o.disabled);
        if self.focused.is_none() {
            self.focused = self.selectable_indices().first().copied();
        }
    }

    fn move_focus(&mut self, delta: i32) {
        let selectable = self.selectable_indices();
        if selectable.is_empty() {
            return;
        }
        let cur = self
            .focused
            .and_then(|i| selectable.iter().position(|&x| x == i))
            .unwrap_or(0);
        let next = if delta >= 0 {
            selectable[(cur + 1).min(selectable.len() - 1)]
        } else {
            selectable[cur.saturating_sub(1)]
        };
        self.focused = Some(next);
    }

    fn draw_option(
        &self,
        renderer: &mut dyn Renderer,
        index: usize,
        row: Rect,
        theme: &Theme,
    ) {
        let opt = &self.options[index];
        let item_disabled = self.disabled || opt.disabled;
        let selected = self.is_selected(index);
        let hovered = self.hovered == Some(index);
        let focused = self.focused == Some(index);

        let circle_y = row.y + (row.height - CIRCLE) / 2.0;
        let circle = Rect::new(row.x, circle_y, CIRCLE, CIRCLE);

        let border = if item_disabled {
            theme.border.with_alpha(0.45)
        } else if focused {
            theme.accent
        } else if hovered {
            theme.fg_subtle
        } else {
            theme.border
        };

        renderer.fill_rect(circle, theme.bg_elevated, Corners::all(CIRCLE / 2.0));
        renderer.stroke_rect(circle, border, 1.5, Corners::all(CIRCLE / 2.0));

        if selected {
            let dot_color = if item_disabled {
                theme.accent.with_alpha(0.45)
            } else {
                theme.accent
            };
            renderer.fill_circle(
                Point::new(circle.x + CIRCLE / 2.0, circle.y + CIRCLE / 2.0),
                INNER / 2.0,
                dot_color,
            );
        }

        if focused && !item_disabled {
            renderer.stroke_rect(
                Rect::new(
                    circle.x - 2.0,
                    circle.y - 2.0,
                    circle.width + 4.0,
                    circle.height + 4.0,
                ),
                theme.accent.with_alpha(0.45),
                2.0,
                Corners::all(CIRCLE / 2.0 + 2.0),
            );
        }

        let color = if item_disabled {
            theme.fg_muted
        } else {
            theme.fg
        };
        let opts = TextOptions {
            font_size: theme.font_size_md,
            color,
            ..Default::default()
        };
        let tx = row.x + CIRCLE + GAP;
        let (_, th, _) = renderer.measure_text(&opt.label, &opts);
        renderer.draw_text(
            &opt.label,
            Point::new(tx, row.y + (row.height - th) / 2.0),
            &opts,
        );
    }
}

impl Widget for RadioGroup {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn focusable(&self) -> bool {
        !self.disabled && self.selectable_indices().len() > 0
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        for i in 0..self.options.len() {
            self.draw_option(renderer, i, self.row_rect(i, bounds), theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }

        match event {
            Event::FocusGained => {
                self.focus_selected_index();
                EventStatus::Ignored
            }
            Event::FocusLost => {
                self.focused = None;
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowDown, .. } | Event::KeyDown { key: Key::ArrowRight, .. } => {
                if self.focused.is_some() || self.focusable() {
                    if self.focused.is_none() {
                        self.focus_selected_index();
                    }
                    self.move_focus(1);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowUp, .. } | Event::KeyDown { key: Key::ArrowLeft, .. } => {
                if self.focused.is_some() || self.focusable() {
                    if self.focused.is_none() {
                        self.focus_selected_index();
                    }
                    self.move_focus(-1);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Enter, .. } | Event::KeyDown { key: Key::Char(' '), .. } => {
                if let Some(i) = self.focused {
                    self.select_index(i);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                self.hovered = self.index_at(*pos, bounds);
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if let Some(i) = self.index_at(*pos, bounds) {
                    self.focused = Some(i);
                    self.select_index(i);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let char_w = theme.font_size_md * 0.6;
        let mut max_w = 0.0_f32;
        for opt in &self.options {
            let label_w = opt.label.chars().count() as f32 * char_w;
            max_w = max_w.max(CIRCLE + GAP + label_w);
        }
        let n = self.options.len().max(1) as f32;
        let h = n * ROW_H + (n - 1.0).max(0.0) * self.spacing;
        (max_w.max(120.0), h)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if self.disabled {
            return CursorStyle::Default;
        }
        if self
            .index_at(Point::new(pos.0, pos.1), bounds)
            .is_some()
        {
            CursorStyle::Pointer
        } else {
            CursorStyle::Default
        }
    }
}

/// Shorthand — build from `(value, label)` pairs.
pub fn radio_group(
    options: Vec<(impl Into<String>, impl Into<String>)>,
    selected: Rc<RefCell<String>>,
) -> RadioGroup {
    RadioGroup::from_pairs(options, selected)
}

/// Shorthand — build from [`RadioGroupOption`] list.
pub fn radio_group_options(
    options: Vec<RadioGroupOption>,
    selected: Rc<RefCell<String>>,
) -> RadioGroup {
    RadioGroup::new(options, selected)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("radio-group-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
