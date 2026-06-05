//! Accordion — collapsible sections (shadcn/ui inspired).
//!
//! ```rust,ignore
//! accordion(vec![
//!     AccordionItem::new("item-1", "Is it accessible?", text("Yes.")),
//!     AccordionItem::new("item-2", "Is it styled?", text("Yes.")),
//! ])
//! .default_open(vec!["item-1"]);
//! ```

use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const HEADER_H: f32 = 44.0;
const CHEVRON_SIZE: f32 = 8.0;

/// One collapsible section.
pub struct AccordionItem {
    pub value: String,
    pub title: String,
    pub content: Box<dyn Widget>,
    pub disabled: bool,
}

impl AccordionItem {
    pub fn new(
        value: impl Into<String>,
        title: impl Into<String>,
        content: impl Widget + 'static,
    ) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            content: Box::new(content),
            disabled: false,
        }
    }

    pub fn disabled(mut self, v: bool) -> Self {
        self.disabled = v;
        self
    }
}

pub struct Accordion {
    id: String,
    items: Vec<AccordionItem>,
    open: Vec<String>,
    multiple: bool,
    collapsible: bool,
    hovered: Option<usize>,
    focused: Option<usize>,
}

impl Accordion {
    pub fn new(items: Vec<AccordionItem>) -> Self {
        Self {
            id: uuid(),
            items,
            open: Vec::new(),
            multiple: false,
            collapsible: true,
            hovered: None,
            focused: None,
        }
    }

    pub fn default_open(mut self, values: Vec<impl Into<String>>) -> Self {
        self.open = values.into_iter().map(|v| v.into()).collect();
        self
    }

    pub fn multiple(mut self, v: bool) -> Self {
        self.multiple = v;
        self
    }

    pub fn collapsible(mut self, v: bool) -> Self {
        self.collapsible = v;
        self
    }

    fn is_open(&self, value: &str) -> bool {
        self.open.iter().any(|v| v == value)
    }

    fn toggle(&mut self, index: usize) {
        let Some(item) = self.items.get(index) else {
            return;
        };
        if item.disabled {
            return;
        }
        let value = item.value.clone();
        if self.is_open(&value) {
            if self.collapsible || self.multiple {
                self.open.retain(|v| v != &value);
            }
        } else if self.multiple {
            self.open.push(value);
        } else {
            self.open = vec![value];
        }
    }

    fn layout_offsets(&self, theme: &Theme) -> Vec<(f32, f32)> {
        let mut out = Vec::with_capacity(self.items.len());
        let mut y = 0.0;
        for item in &self.items {
            out.push((y, if self.is_open(&item.value) {
                item.content.intrinsic_size(theme).1
            } else {
                0.0
            }));
            y += HEADER_H + out.last().map(|(_, h)| *h).unwrap_or(0.0);
        }
        out
    }

    fn header_rect(&self, index: usize, bounds: Rect, theme: &Theme) -> Rect {
        let y = bounds.y + self.layout_offsets(theme)[index].0;
        Rect::new(bounds.x, y, bounds.width, HEADER_H)
    }

    fn content_rect(&self, index: usize, bounds: Rect, theme: &Theme) -> Rect {
        let (y_off, h) = self.layout_offsets(theme)[index];
        if h <= 0.0 {
            return Rect::new(bounds.x, bounds.y, 0.0, 0.0);
        }
        Rect::new(
            bounds.x,
            bounds.y + y_off + HEADER_H,
            bounds.width,
            h,
        )
    }

    fn index_at(&self, pos: Point, bounds: Rect, theme: &Theme) -> Option<usize> {
        for i in 0..self.items.len() {
            if self.header_rect(i, bounds, theme).contains(pos.x, pos.y) {
                return Some(i);
            }
        }
        None
    }

    fn draw_chevron(&self, renderer: &mut dyn Renderer, header: Rect, open: bool, theme: &Theme) {
        let cx = header.x + header.width - 20.0;
        let cy = header.y + header.height / 2.0;
        let color = theme.fg_muted;
        if open {
            renderer.draw_line(
                Point::new(cx - CHEVRON_SIZE, cy - 2.0),
                Point::new(cx, cy + 4.0),
                color,
                1.5,
            );
            renderer.draw_line(
                Point::new(cx, cy + 4.0),
                Point::new(cx + CHEVRON_SIZE, cy - 2.0),
                color,
                1.5,
            );
        } else {
            renderer.draw_line(
                Point::new(cx - 4.0, cy - CHEVRON_SIZE),
                Point::new(cx + 4.0, cy),
                color,
                1.5,
            );
            renderer.draw_line(
                Point::new(cx + 4.0, cy),
                Point::new(cx - 4.0, cy + CHEVRON_SIZE),
                color,
                1.5,
            );
        }
    }
}

impl Widget for Accordion {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    fn focusable(&self) -> bool {
        true
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        for (i, item) in self.items.iter().enumerate() {
            let header = self.header_rect(i, bounds, theme);
            let open = self.is_open(&item.value);

            if i > 0 {
                renderer.fill_rect(
                    Rect::new(bounds.x, header.y, bounds.width, 1.0),
                    theme.border,
                    Corners::ZERO,
                );
            }

            if self.hovered == Some(i) && !item.disabled {
                renderer.fill_rect(
                    header,
                    theme.bg_elevated.with_alpha(0.35),
                    Corners::ZERO,
                );
            }

            let title_color = if item.disabled {
                theme.fg_muted
            } else {
                theme.fg
            };
            let opts = TextOptions {
                font_size: theme.font_size_md,
                color: title_color,
                ..Default::default()
            };
            let (_, th, _) = renderer.measure_text(&item.title, &opts);
            renderer.draw_text(
                &item.title,
                Point::new(
                    header.x,
                    header.y + (header.height - th) / 2.0,
                ),
                &opts,
            );
            self.draw_chevron(renderer, header, open, theme);

            if open {
                let content_bounds = self.content_rect(i, bounds, theme);
                item.content.draw(renderer, content_bounds, theme);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        match event {
            Event::FocusGained => EventStatus::Ignored,
            Event::FocusLost => {
                self.focused = None;
                EventStatus::Ignored
            }
            Event::KeyDown { key: Key::Enter, .. } | Event::KeyDown { key: Key::Char(' '), .. } => {
                if let Some(i) = self.focused {
                    self.toggle(i);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                self.hovered = self.index_at(*pos, bounds, &theme);
                EventStatus::Ignored
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if let Some(i) = self.index_at(*pos, bounds, &theme) {
                    self.focused = Some(i);
                    self.toggle(i);
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            _ => {
                let open_indices: Vec<(usize, Rect)> = (0..self.items.len())
                    .filter_map(|i| {
                        let value = self.items[i].value.clone();
                        if self.is_open(&value) {
                            Some((i, self.content_rect(i, bounds, &theme)))
                        } else {
                            None
                        }
                    })
                    .collect();
                for (i, cb) in open_indices {
                    if self.items[i].content.handle_event(event, cb) == EventStatus::Consumed {
                        return EventStatus::Consumed;
                    }
                }
                EventStatus::Ignored
            }
        }
    }

    fn layout_children<'a>(&'a self, bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        let mut out = Vec::new();
        for (i, item) in self.items.iter().enumerate() {
            if self.is_open(&item.value) {
                out.push((
                    item.content.as_ref() as &dyn Widget,
                    self.content_rect(i, bounds, theme),
                ));
            }
        }
        out
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let mut max_w = 0.0_f32;
        let mut total_h = 0.0_f32;
        for item in &self.items {
            let (cw, ch) = item.content.intrinsic_size(theme);
            max_w = max_w.max(cw);
            total_h += HEADER_H;
            if self.is_open(&item.value) {
                total_h += ch;
            }
        }
        (max_w.max(280.0), total_h.max(HEADER_H))
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let theme = Theme::default();
        if self
            .index_at(Point::new(pos.0, pos.1), bounds, &theme)
            .is_some()
        {
            CursorStyle::Pointer
        } else {
            CursorStyle::Default
        }
    }
}

pub fn accordion(items: Vec<AccordionItem>) -> Accordion {
    Accordion::new(items)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("accordion-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
