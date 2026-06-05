//! Tabs — tab list with panel content (shadcn/ui inspired).
//!
//! Distinct from [`TabView`](crate::widget::tab_view::TabView) which switches pages
//! programmatically (e.g. sidebar navigation). `Tabs` renders clickable triggers.
//!
//! ```rust,ignore
//! tabs(vec![
//!     ("Account", account_panel),
//!     ("Password", password_panel),
//! ])
//! .active("Account");
//! ```

use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const TAB_BAR_H: f32 = 40.0;
const TAB_PAD_X: f32 = 12.0;

pub struct Tabs {
    id: String,
    panels: Vec<(String, Box<dyn Widget>)>,
    active: String,
    hovered: Option<usize>,
    focused: Option<usize>,
    on_change: Option<Box<dyn Fn(&str)>>,
}

impl Tabs {
    pub fn new(panels: Vec<(String, Box<dyn Widget>)>) -> Self {
        let first = panels.first().map(|(k, _)| k.clone()).unwrap_or_default();
        Self {
            id: uuid(),
            panels,
            active: first,
            hovered: None,
            focused: None,
            on_change: None,
        }
    }

    pub fn active(mut self, key: impl Into<String>) -> Self {
        self.active = key.into();
        self
    }

    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn set_active(&mut self, key: impl Into<String>) {
        self.active = key.into();
    }

    fn tab_widths(&self, theme: &Theme) -> Vec<f32> {
        self.panels
            .iter()
            .map(|(label, _)| {
                let opts = TextOptions {
                    font_size: theme.font_size_sm,
                    color: theme.fg,
                    ..Default::default()
                };
                let (tw, _, _) = measure_label(label, &opts);
                tw + TAB_PAD_X * 2.0
            })
            .collect()
    }

    fn tab_rects(&self, bounds: Rect, theme: &Theme) -> Vec<Rect> {
        let widths = self.tab_widths(theme);
        let mut x = bounds.x;
        widths
            .into_iter()
            .map(|w| {
                let r = Rect::new(x, bounds.y, w, TAB_BAR_H);
                x += w;
                r
            })
            .collect()
    }

    fn content_bounds(&self, bounds: Rect) -> Rect {
        Rect::new(
            bounds.x,
            bounds.y + TAB_BAR_H,
            bounds.width,
            (bounds.height - TAB_BAR_H).max(0.0),
        )
    }

    fn select(&mut self, label: &str) {
        if self.active == label {
            return;
        }
        self.active = label.to_string();
        if let Some(f) = &self.on_change {
            f(label);
        }
    }

    fn tab_index_at(&self, pos: Point, bounds: Rect, theme: &Theme) -> Option<usize> {
        for (i, rect) in self.tab_rects(bounds, theme).iter().enumerate() {
            if rect.contains(pos.x, pos.y) {
                return Some(i);
            }
        }
        None
    }
}

impl Widget for Tabs {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn focusable(&self) -> bool {
        !self.panels.is_empty()
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let tab_rects = self.tab_rects(bounds, theme);

        // Tab bar bottom border
        renderer.fill_rect(
            Rect::new(bounds.x, bounds.y + TAB_BAR_H - 1.0, bounds.width, 1.0),
            theme.border,
            Corners::ZERO,
        );

        for (i, ((label, _), tab)) in self.panels.iter().zip(tab_rects.iter()).enumerate() {
            let is_active = *label == self.active;
            let is_hovered = self.hovered == Some(i);

            if is_active {
                renderer.fill_rect(
                    Rect::new(tab.x, tab.y + TAB_BAR_H - 2.0, tab.width, 2.0),
                    theme.accent,
                    Corners::ZERO,
                );
            } else if is_hovered {
                renderer.fill_rect(
                    *tab,
                    theme.bg_elevated.with_alpha(0.35),
                    Corners::ZERO,
                );
            }

            let color = if is_active {
                theme.fg
            } else {
                theme.fg_muted
            };
            let opts = TextOptions {
                font_size: theme.font_size_sm,
                color,
                bold: is_active,
                ..Default::default()
            };
            let (tw, th, _) = measure_label(label, &opts);
            renderer.draw_text(
                label,
                Point::new(
                    tab.x + (tab.width - tw) / 2.0,
                    tab.y + (TAB_BAR_H - th) / 2.0,
                ),
                &opts,
            );
        }

        let content = self.content_bounds(bounds);
        if let Some((_, panel)) = self.panels.iter().find(|(k, _)| k == &self.active) {
            panel.draw(renderer, content, theme);
        }
    }

    fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let content = self.content_bounds(bounds);
        if let Some((_, panel)) = self.panels.iter().find(|(k, _)| k == &self.active) {
            panel.draw_overlay(renderer, content, theme);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let _tab_rects = self.tab_rects(bounds, &theme);
        let content = self.content_bounds(bounds);

        match event {
            Event::FocusGained => EventStatus::Ignored,
            Event::FocusLost => {
                self.focused = None;
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowRight, .. } => {
                if self.focused.is_some() || self.focusable() {
                    let idx = self
                        .panels
                        .iter()
                        .position(|(k, _)| k == &self.active)
                        .unwrap_or(0);
                    let next = (idx + 1).min(self.panels.len().saturating_sub(1));
                    let label = self.panels[next].0.clone();
                    self.select(&label);
                    self.focused = Some(next);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::ArrowLeft, .. } => {
                if self.focused.is_some() || self.focusable() {
                    let idx = self
                        .panels
                        .iter()
                        .position(|(k, _)| k == &self.active)
                        .unwrap_or(0);
                    let next = idx.saturating_sub(1);
                    let label = self.panels[next].0.clone();
                    self.select(&label);
                    self.focused = Some(next);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                self.hovered = self.tab_index_at(*pos, bounds, &theme);
                if content.contains(pos.x, pos.y) {
                    if let Some((_, panel)) = self.panels.iter_mut().find(|(k, _)| k == &self.active)
                    {
                        return panel.handle_event(event, content);
                    }
                }
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if let Some(i) = self.tab_index_at(*pos, bounds, &theme) {
                    let label = self.panels[i].0.clone();
                    self.focused = Some(i);
                    self.select(&label);
                    return EventStatus::Consumed;
                }
                if content.contains(pos.x, pos.y) {
                    if let Some((_, panel)) = self.panels.iter_mut().find(|(k, _)| k == &self.active)
                    {
                        return panel.handle_event(event, content);
                    }
                }
                EventStatus::Ignored
            }
            _ => {
                if let Some((_, panel)) = self.panels.iter_mut().find(|(k, _)| k == &self.active) {
                    return panel.handle_event(event, content);
                }
                EventStatus::Ignored
            }
        }
    }

    fn layout_children<'a>(&'a self, bounds: Rect, _theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        let content = self.content_bounds(bounds);
        if let Some((_, panel)) = self.panels.iter().find(|(k, _)| k == &self.active) {
            vec![(panel.as_ref() as &dyn Widget, content)]
        } else {
            vec![]
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let tab_w: f32 = self.tab_widths(theme).iter().sum();
        let mut max_panel_h = 0.0_f32;
        let mut max_panel_w = 0.0_f32;
        for (_, panel) in &self.panels {
            let (w, h) = panel.intrinsic_size(theme);
            max_panel_w = max_panel_w.max(w);
            max_panel_h = max_panel_h.max(h);
        }
        (tab_w.max(max_panel_w).max(320.0), TAB_BAR_H + max_panel_h)
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        if self
            .tab_index_at(Point::new(pos.0, pos.1), bounds, &theme)
            .is_some()
        {
            return CursorStyle::Pointer;
        }
        let content = self.content_bounds(bounds);
        if let Some((_, panel)) = self.panels.iter().find(|(k, _)| k == &self.active) {
            return panel.cursor_at(pos, content);
        }
        CursorStyle::Default
    }
}

fn measure_label(label: &str, opts: &TextOptions) -> (f32, f32, f32) {
    let char_w = opts.font_size * 0.55;
    let w = label.chars().count() as f32 * char_w;
    let h = opts.font_size * 1.2;
    (w, h, h)
}

pub fn tabs(panels: Vec<(String, Box<dyn Widget>)>) -> Tabs {
    Tabs::new(panels)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("tabs-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
