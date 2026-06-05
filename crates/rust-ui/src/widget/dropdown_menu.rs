//! Dropdown Menu — action menu anchored to a trigger (shadcn/ui inspired).
//!
//! ```rust,ignore
//! dropdown_menu(
//!     button("Open"),
//!     vec![
//!         SelectEntry::item("Profile"),
//!         SelectEntry::item("Settings"),
//!         SelectEntry::separator(),
//!         SelectEntry::item("Sign out"),
//!     ],
//! )
//! .on_select(|label| println!("{label}"))
//! .width(200.0);
//! ```

use std::cell::Cell;

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::overlay::{place_popup, viewport_around_anchor, PopoverAlign, PopoverSide};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::select::SelectEntry;
use crate::widget::Widget;

const GAP: f32 = 4.0;
const MENU_PAD: f32 = 4.0;
const MAX_MENU_H: f32 = 320.0;
const MIN_MENU_W: f32 = 180.0;

pub struct DropdownMenu {
    id: String,
    trigger: Box<dyn Widget>,
    entries: Vec<SelectEntry>,
    open: bool,
    disabled: bool,
    focused: bool,
    highlighted: Option<usize>,
    menu_width: Option<f32>,
    side: PopoverSide,
    align: PopoverAlign,
    scroll_y: Cell<f32>,
    on_select: Option<Box<dyn Fn(&str)>>,
    last_bounds: Cell<(f32, f32, f32, f32)>,
    last_trigger: Cell<(f32, f32, f32, f32)>,
    last_panel: Cell<(f32, f32, f32, f32)>,
}

impl DropdownMenu {
    pub fn new(trigger: impl Widget + 'static, entries: Vec<SelectEntry>) -> Self {
        Self {
            id: uuid(),
            trigger: Box::new(trigger),
            entries,
            open: false,
            disabled: false,
            focused: false,
            highlighted: None,
            menu_width: None,
            side: PopoverSide::Bottom,
            align: PopoverAlign::Start,
            scroll_y: Cell::new(0.0),
            on_select: None,
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_trigger: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_panel: Cell::new((0.0, 0.0, 0.0, 0.0)),
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.menu_width = Some(w);
        self
    }

    pub fn side(mut self, side: PopoverSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: PopoverAlign) -> Self {
        self.align = align;
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

    pub fn on_select(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    fn trigger_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let (tw, th) = self.trigger.intrinsic_size(theme);
        Rect::new(bounds.x, bounds.y, tw, th)
    }

    fn menu_width(&self, trigger: Rect) -> f32 {
        self.menu_width
            .unwrap_or(trigger.width.max(MIN_MENU_W))
    }

    fn content_height(&self) -> f32 {
        self.entries.iter().map(|e| e.row_height()).sum::<f32>() + MENU_PAD * 2.0
    }

    fn menu_height(&self) -> f32 {
        self.content_height().min(MAX_MENU_H)
    }

    fn panel_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let trigger = self.trigger_rect(bounds, theme);
        let pw = self.menu_width(trigger);
        let ph = self.menu_height();
        let viewport = viewport_around_anchor(trigger, 24.0);
        place_popup(trigger, pw, ph, viewport, self.side, self.align, GAP)
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

    fn open_menu(&mut self) {
        if self.selectable_indices().is_empty() {
            self.open = false;
            return;
        }
        self.open = true;
        self.highlighted = self.selectable_indices().first().copied();
        self.ensure_highlight_visible();
    }

    fn activate(&mut self, idx: usize) {
        if let Some(SelectEntry::Item { label, .. }) = self.entries.get(idx) {
            let label = label.clone();
            if let Some(f) = &self.on_select {
                f(&label);
            }
        }
        self.close();
    }

    fn ensure_highlight_visible(&self) {
        let Some(hl) = self.highlighted else {
            return;
        };
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

    fn panel_contains(&self, pos: Point) -> bool {
        let (x, y, w, h) = self.last_panel.get();
        w > 0.0 && Rect::new(x, y, w, h).contains(pos.x, pos.y)
    }

    fn draw_menu(&self, renderer: &mut dyn Renderer, menu: Rect, theme: &Theme) {
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
                    let hl = self.highlighted == Some(i);
                    if hl && !disabled {
                        renderer.fill_rect(
                            row,
                            theme.accent.with_alpha(0.12),
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

impl Widget for DropdownMenu {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn focusable(&self) -> bool {
        !self.disabled
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
        if !self.open {
            self.last_panel.set((0.0, 0.0, 0.0, 0.0));
            return;
        }

        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = if bw > 0.0 {
            Rect::new(bx, by, bw, bh)
        } else {
            bounds
        };

        let menu = self.panel_rect(bounds, theme);
        self.last_panel
            .set((menu.x, menu.y, menu.width, menu.height));
        self.draw_menu(renderer, menu, theme);
    }

    fn hit_test(&self, pos: (f32, f32), bounds: Rect, theme: &Theme) -> bool {
        if self.trigger_rect(bounds, theme).contains(pos.0, pos.1) {
            return true;
        }
        if self.open && self.panel_contains(Point::new(pos.0, pos.1)) {
            return true;
        }
        let _ = theme;
        false
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if self.disabled {
            return EventStatus::Ignored;
        }

        let theme = Theme::default();
        let trigger = self.trigger_rect(bounds, &theme);
        let menu = self.panel_rect(bounds, &theme);

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
                        self.activate(i);
                    } else {
                        self.close();
                    }
                } else {
                    self.open_menu();
                }
                EventStatus::Consumed
            }
            Event::KeyDown { key: Key::ArrowDown, .. } => {
                if !self.focused {
                    return EventStatus::Ignored;
                }
                if !self.open {
                    self.open_menu();
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
                    self.open_menu();
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
                if self.open {
                    self.highlighted = self.entry_at(*pos, menu);
                }
                if self.open && self.panel_contains(*pos) {
                    return EventStatus::Ignored;
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if self.open && self.panel_contains(*pos) {
                    if let Some(i) = self.entry_at(*pos, menu) {
                        self.activate(i);
                    }
                    return EventStatus::Consumed;
                }
                if trigger.contains(pos.x, pos.y) {
                    if self.open {
                        self.close();
                    } else {
                        self.open_menu();
                    }
                    self.trigger.handle_event(event, trigger);
                    return EventStatus::Consumed;
                }
                if self.open {
                    self.close();
                    return EventStatus::Consumed;
                }
                self.trigger.handle_event(event, trigger)
            }
            Event::MouseUp { pos, .. } => {
                if self.open && self.panel_contains(*pos) {
                    return EventStatus::Consumed;
                }
                self.trigger.handle_event(event, trigger)
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
            _ => self.trigger.handle_event(event, trigger),
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        self.trigger.intrinsic_size(theme)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        if self.disabled {
            return CursorStyle::Default;
        }
        if self.open && self.panel_contains(Point::new(pos.0, pos.1)) {
            return CursorStyle::Pointer;
        }
        let trigger = self.trigger_rect(bounds, &theme);
        self.trigger.cursor_at(pos, trigger)
    }
}

/// Shorthand constructor.
pub fn dropdown_menu(trigger: impl Widget + 'static, entries: Vec<SelectEntry>) -> DropdownMenu {
    DropdownMenu::new(trigger, entries)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("dropdown-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
