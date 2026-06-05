//! Select widget — dropdown option picker (shadcn/ui inspired).
//!
//! ```rust,ignore
//! select(vec!["Apple", "Banana", "Cherry"])
//!     .placeholder("Select a fruit")
//!     .width(180.0);
//!
//! select_entries(vec![
//!     SelectEntry::label("Fruits"),
//!     SelectEntry::item("Apple"),
//!     SelectEntry::item("Banana"),
//!     SelectEntry::separator(),
//!     SelectEntry::label("Vegetables"),
//!     SelectEntry::item("Carrot"),
//! ]);
//! ```

use std::cell::Cell;

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const TRIGGER_H: f32 = 36.0;
const TRIGGER_PAD_X: f32 = 12.0;
const MENU_GAP: f32 = 4.0;
const MENU_PAD: f32 = 4.0;
const ITEM_H: f32 = 32.0;
const LABEL_H: f32 = 28.0;
const SEPARATOR_H: f32 = 9.0;
const MAX_MENU_H: f32 = 240.0;
const DEFAULT_WIDTH: f32 = 180.0;

/// A single row inside a select menu.
#[derive(Debug, Clone)]
pub enum SelectEntry {
    /// Non-interactive group heading (like `SelectLabel`).
    Label(String),
    /// Selectable option (like `SelectItem`).
    Item {
        label: String,
        disabled: bool,
    },
    /// Horizontal rule between groups (like `SelectSeparator`).
    Separator,
}

impl SelectEntry {
    pub fn item(label: impl Into<String>) -> Self {
        Self::Item {
            label: label.into(),
            disabled: false,
        }
    }

    pub fn item_disabled(label: impl Into<String>) -> Self {
        Self::Item {
            label: label.into(),
            disabled: true,
        }
    }

    pub fn label(text: impl Into<String>) -> Self {
        Self::Label(text.into())
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    pub(crate) fn row_height(&self) -> f32 {
        match self {
            Self::Label(_) => LABEL_H,
            Self::Item { .. } => ITEM_H,
            Self::Separator => SEPARATOR_H,
        }
    }
}

pub struct Select {
    id: String,
    entries: Vec<SelectEntry>,
    placeholder: String,
    /// Index into `entries` for the selected `Item`.
    selected: Option<usize>,
    open: bool,
    disabled: bool,
    invalid: bool,
    hovered: bool,
    focused: bool,
    highlighted: Option<usize>,
    width: Option<f32>,
    scroll_y: Cell<f32>,
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl Select {
    pub fn new(options: Vec<String>) -> Self {
        Self::from_entries(
            options
                .into_iter()
                .map(SelectEntry::item)
                .collect(),
        )
    }

    pub fn from_entries(entries: Vec<SelectEntry>) -> Self {
        Self {
            id: uuid(),
            entries,
            placeholder: "Select…".to_string(),
            selected: None,
            open: false,
            disabled: false,
            invalid: false,
            hovered: false,
            focused: false,
            highlighted: None,
            width: None,
            scroll_y: Cell::new(0.0),
            on_change: None,
        }
    }

    pub fn placeholder(mut self, s: impl Into<String>) -> Self {
        self.placeholder = s.into();
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        if v {
            self.open = false;
            self.highlighted = None;
        }
        self
    }

    pub fn invalid(mut self, v: bool) -> Self {
        self.invalid = v;
        self
    }

    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Pre-select by item index among selectable `Item` entries (0 = first item).
    pub fn selected(mut self, item_index: usize) -> Self {
        let selectable = self.selectable_indices();
        if let Some(&idx) = selectable.get(item_index) {
            self.selected = Some(idx);
        }
        self
    }

    /// Pre-select by matching item label.
    pub fn selected_value(mut self, value: &str) -> Self {
        if let Some(idx) = self.entries.iter().position(|e| {
            matches!(e, SelectEntry::Item { label, disabled: false } if label == value)
        }) {
            self.selected = Some(idx);
        }
        self
    }

    fn trigger_width(&self, bounds: Rect) -> f32 {
        self.width.unwrap_or(DEFAULT_WIDTH).min(bounds.width.max(DEFAULT_WIDTH))
    }

    fn control_rect(&self, bounds: Rect) -> Rect {
        let w = self.trigger_width(bounds);
        Rect::new(bounds.x, bounds.y, w, TRIGGER_H)
    }

    fn content_height(&self) -> f32 {
        self.entries.iter().map(|e| e.row_height()).sum::<f32>() + MENU_PAD * 2.0
    }

    fn menu_height(&self) -> f32 {
        self.content_height().min(MAX_MENU_H)
    }

    fn menu_rect(&self, control: Rect) -> Rect {
        Rect::new(
            control.x,
            control.y + control.height + MENU_GAP,
            control.width,
            self.menu_height(),
        )
    }

    fn selectable_indices(&self) -> Vec<usize> {
        self.entries
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                SelectEntry::Item { disabled: false, .. } => Some(i),
                _ => None,
            })
            .collect()
    }

    fn entry_at(&self, pos: Point, menu: Rect) -> Option<usize> {
        if !menu.contains(pos.x, pos.y) {
            return None;
        }
        let scroll = self.scroll_y.get();
        let mut y = menu.y + MENU_PAD - scroll;
        for (i, e) in self.entries.iter().enumerate() {
            let h = e.row_height();
            let row = Rect::new(menu.x + MENU_PAD, y, menu.width - MENU_PAD * 2.0, h);
            if row.contains(pos.x, pos.y) {
                return match e {
                    SelectEntry::Item { disabled: false, .. } => Some(i),
                    _ => None,
                };
            }
            y += h;
        }
        None
    }

    fn close(&mut self) {
        self.open = false;
        self.highlighted = None;
        self.scroll_y.set(0.0);
    }

    fn open(&mut self) {
        if self.selectable_indices().is_empty() {
            self.open = false;
            return;
        }
        self.open = true;
        self.highlighted = self
            .selected
            .or_else(|| self.selectable_indices().first().copied());
        self.ensure_highlight_visible();
    }

    fn commit(&mut self, idx: usize) {
        if let Some(SelectEntry::Item { label, .. }) = self.entries.get(idx) {
            let label = label.clone();
            self.selected = Some(idx);
            if let Some(f) = &self.on_change {
                f(&label);
            }
        }
        self.close();
    }

    fn ensure_highlight_visible(&self) {
        let Some(hl) = self.highlighted else { return };
        let menu_h = self.menu_height() - MENU_PAD * 2.0;
        let mut y = 0.0_f32;
        for (i, e) in self.entries.iter().enumerate() {
            if i == hl {
                let row_h = e.row_height();
                let scroll = self.scroll_y.get();
                if y < scroll {
                    self.scroll_y.set(y);
                } else if y + row_h > scroll + menu_h {
                    self.scroll_y.set((y + row_h - menu_h).max(0.0));
                }
                return;
            }
            y += e.row_height();
        }
    }

    fn selected_label(&self) -> Option<&str> {
        self.selected.and_then(|i| match self.entries.get(i) {
            Some(SelectEntry::Item { label, .. }) => Some(label.as_str()),
            _ => None,
        })
    }

    fn draw_trigger(&self, renderer: &mut dyn Renderer, control: Rect, theme: &Theme) {
        let border = if self.disabled {
            theme.border.with_alpha(0.45)
        } else if self.invalid {
            theme.danger
        } else if self.focused {
            theme.accent
        } else if self.hovered {
            theme.fg_subtle
        } else {
            theme.border
        };

        renderer.fill_rect(control, theme.bg_elevated, Corners::all(theme.radius_md));
        renderer.stroke_rect(control, border, 1.0, Corners::all(theme.radius_md));

        // Subtle inner highlight (shadow-sm feel)
        renderer.stroke_rect(
            Rect::new(control.x + 0.5, control.y + 0.5, control.width - 1.0, control.height - 1.0),
            Color::WHITE.with_alpha(0.03),
            1.0,
            Corners::all(theme.radius_md - 0.5),
        );

        let (label, label_color) = match self.selected_label() {
            Some(s) => (s, theme.fg),
            None => (self.placeholder.as_str(), theme.fg_muted),
        };
        let opts = TextOptions {
            font_size: theme.font_size_md,
            color: if self.disabled {
                theme.fg_muted
            } else {
                label_color
            },
            ..Default::default()
        };
        let (_, th, _) = renderer.measure_text(label, &opts);
        renderer.draw_text(
            label,
            Point::new(
                control.x + TRIGGER_PAD_X,
                control.y + (control.height - th) / 2.0,
            ),
            &opts,
        );

        // Chevron down (lucide-style)
        let cx = control.x + control.width - 16.0;
        let cy = control.y + control.height / 2.0;
        let cc = theme.fg_muted;
        renderer.draw_line(Point::new(cx - 3.5, cy - 1.5), Point::new(cx, cy + 2.0), cc, 1.5);
        renderer.draw_line(Point::new(cx, cy + 2.0), Point::new(cx + 3.5, cy - 1.5), cc, 1.5);

        if self.focused && !self.disabled {
            renderer.stroke_rect(
                Rect::new(
                    control.x - 2.0,
                    control.y - 2.0,
                    control.width + 4.0,
                    control.height + 4.0,
                ),
                theme.accent.with_alpha(if self.invalid { 0.25 } else { 0.45 }),
                2.0,
                Corners::all(theme.radius_md + 2.0),
            );
        }
    }

    fn draw_menu(&self, renderer: &mut dyn Renderer, menu: Rect, theme: &Theme) {
        // Popover shadow
        renderer.fill_rect(
            Rect::new(menu.x + 1.0, menu.y + 2.0, menu.width, menu.height),
            Color::BLACK.with_alpha(0.18),
            Corners::all(theme.radius_md),
        );

        renderer.fill_rect(menu, theme.bg_surface, Corners::all(theme.radius_md));
        renderer.stroke_rect(menu, theme.border, 1.0, Corners::all(theme.radius_md));

        let scroll = self.scroll_y.get();
        renderer.push_clip(menu);
        renderer.push_offset(0.0, -scroll);

        let mut y = menu.y + MENU_PAD;
        for (i, entry) in self.entries.iter().enumerate() {
            let row_w = menu.width - MENU_PAD * 2.0;
            let row = Rect::new(menu.x + MENU_PAD, y, row_w, entry.row_height());

            match entry {
                SelectEntry::Label(text) => {
                    let opts = TextOptions {
                        font_size: theme.font_size_sm,
                        color: theme.fg_muted,
                        bold: true,
                        ..Default::default()
                    };
                    let (_, th, _) = renderer.measure_text(text, &opts);
                    renderer.draw_text(
                        text,
                        Point::new(row.x + 8.0, row.y + (row.height - th) / 2.0),
                        &opts,
                    );
                }
                SelectEntry::Item { label, disabled } => {
                    let active = self.selected == Some(i);
                    let hl = self.highlighted == Some(i);

                    if hl && !disabled {
                        renderer.fill_rect(
                            row,
                            theme.accent.with_alpha(0.12),
                            Corners::all(theme.radius_sm),
                        );
                    } else if active {
                        renderer.fill_rect(
                            row,
                            theme.bg_elevated.with_alpha(0.5),
                            Corners::all(theme.radius_sm),
                        );
                    }

                    let color = if *disabled {
                        theme.fg_muted.with_alpha(0.6)
                    } else {
                        theme.fg
                    };
                    let opts = TextOptions {
                        font_size: theme.font_size_md,
                        color,
                        ..Default::default()
                    };
                    let (_, th, _) = renderer.measure_text(label, &opts);
                    renderer.draw_text(
                        label,
                        Point::new(row.x + 8.0, row.y + (row.height - th) / 2.0),
                        &opts,
                    );

                    if active {
                        let check = TextOptions {
                            font_size: theme.font_size_sm,
                            color: theme.accent,
                            ..Default::default()
                        };
                        renderer.draw_text(
                            "✓",
                            Point::new(row.x + row.width - 20.0, row.y + (row.height - th) / 2.0),
                            &check,
                        );
                    }
                }
                SelectEntry::Separator => {
                    let line_y = row.y + row.height / 2.0;
                    renderer.fill_rect(
                        Rect::new(row.x + 4.0, line_y, row.width - 8.0, 1.0),
                        theme.border,
                        Corners::ZERO,
                    );
                }
            }
            y += entry.row_height();
        }

        renderer.pop_offset();
        renderer.pop_clip();
    }
}

impl Widget for Select {
    fn id(&self) -> &str {
        &self.id
    }

    fn focusable(&self) -> bool {
        !self.disabled
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        self.draw_trigger(renderer, self.control_rect(bounds), theme);
    }

    fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        if !self.open {
            return;
        }
        let control = self.control_rect(bounds);
        self.draw_menu(renderer, self.menu_rect(control), theme);
    }

    fn hit_test(&self, pos: (f32, f32), bounds: Rect, _theme: &Theme) -> bool {
        let control = self.control_rect(bounds);
        if control.contains(pos.0, pos.1) {
            return true;
        }
        if self.open {
            return self.menu_rect(control).contains(pos.0, pos.1);
        }
        false
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }
        let control = self.control_rect(bounds);
        let menu = self.menu_rect(control);

        match event {
            Event::FocusGained => {
                self.focused = true;
                EventStatus::Ignored
            }
            Event::FocusLost => {
                self.focused = false;
                self.close();
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Escape, .. } => {
                if self.open {
                    self.close();
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Enter, .. } | Event::KeyDown { key: Key::Char(' '), .. } => {
                if !self.focused {
                    return EventStatus::Ignored;
                }
                if self.open {
                    if let Some(i) = self.highlighted {
                        self.commit(i);
                    } else {
                        self.close();
                    }
                } else {
                    self.open();
                }
                EventStatus::Consumed
            }
            Event::KeyDown { key: Key::ArrowDown, .. } => {
                if !self.focused {
                    return EventStatus::Ignored;
                }
                if !self.open {
                    self.open();
                    return EventStatus::Consumed;
                }
                let selectable = self.selectable_indices();
                if selectable.is_empty() {
                    return EventStatus::Ignored;
                }
                let cur = self.highlighted.unwrap_or(selectable[0]);
                let pos = selectable.iter().position(|&x| x == cur).unwrap_or(0);
                let next = selectable[(pos + 1).min(selectable.len() - 1)];
                self.highlighted = Some(next);
                self.ensure_highlight_visible();
                EventStatus::Consumed
            }
            Event::KeyDown { key: Key::ArrowUp, .. } => {
                if !self.focused {
                    return EventStatus::Ignored;
                }
                if !self.open {
                    self.open();
                    return EventStatus::Consumed;
                }
                let selectable = self.selectable_indices();
                if selectable.is_empty() {
                    return EventStatus::Ignored;
                }
                let cur = self.highlighted.unwrap_or(selectable[0]);
                let pos = selectable.iter().position(|&x| x == cur).unwrap_or(0);
                let next = selectable[pos.saturating_sub(1)];
                self.highlighted = Some(next);
                self.ensure_highlight_visible();
                EventStatus::Consumed
            }
            Event::MouseMove { pos } => {
                self.hovered = control.contains(pos.x, pos.y);
                if self.open {
                    self.highlighted = self.entry_at(*pos, menu);
                }
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if control.contains(pos.x, pos.y) {
                    if self.open {
                        self.close();
                    } else {
                        self.open();
                    }
                    return EventStatus::Consumed;
                }
                if self.open && menu.contains(pos.x, pos.y) {
                    if let Some(i) = self.entry_at(*pos, menu) {
                        self.commit(i);
                    }
                    return EventStatus::Consumed;
                }
                if self.open {
                    self.close();
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::Scroll { pos, delta_y, .. } => {
                if self.open && menu.contains(pos.x, pos.y) {
                    let max_scroll = (self.content_height() - self.menu_height()).max(0.0);
                    let next = (self.scroll_y.get() - delta_y).clamp(0.0, max_scroll);
                    self.scroll_y.set(next);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        (self.width.unwrap_or(DEFAULT_WIDTH), TRIGGER_H)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        if self.disabled {
            CursorStyle::Default
        } else if self.hit_test(pos, bounds, &Theme::default()) {
            CursorStyle::Pointer
        } else {
            CursorStyle::Default
        }
    }
}

/// Shorthand — flat list of options.
pub fn select<T: Into<String>>(options: Vec<T>) -> Select {
    Select::new(options.into_iter().map(|s| s.into()).collect())
}

/// Shorthand — grouped entries with labels and separators.
pub fn select_entries(entries: Vec<SelectEntry>) -> Select {
    Select::from_entries(entries)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("select-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
