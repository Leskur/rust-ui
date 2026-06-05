//! Dialog / Modal — centered overlay with backdrop, focus trap, and keyboard dismiss.
//!
//! Place a `Dialog` as the top-most child of a [`Stack`](crate::widget::stack::Stack)
//! so it paints above the rest of the UI.
//!
//! ```rust,ignore
//! let open = Rc::new(RefCell::new(false));
//!
//! stack![
//!     main_content,
//!     dialog(column![
//!         text("Delete this item?"),
//!         row![button("Cancel"), button("Delete")],
//!     ])
//!     .shared_open(open.clone())
//!     .title("Confirm"),
//! ]
//! ```

use std::cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;

use crate::color::Color;
use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

const PANEL_PADDING: f32 = 24.0;
const TITLE_HEIGHT: f32 = 44.0;
const DEFAULT_WIDTH: f32 = 420.0;

pub struct Dialog {
    id: String,
    open: Rc<RefCell<bool>>,
    title: Option<String>,
    content: Box<dyn Widget>,
    width: f32,
    close_on_backdrop: bool,
    close_on_escape: bool,
    on_close: Option<Box<dyn Fn()>>,
    last_bounds: Cell<(f32, f32, f32, f32)>,
    last_panel: Cell<(f32, f32, f32, f32)>,
}

impl Dialog {
    pub fn new(content: impl Widget + 'static) -> Self {
        Self {
            id: uuid(),
            open: Rc::new(RefCell::new(false)),
            title: None,
            content: Box::new(content),
            width: DEFAULT_WIDTH,
            close_on_backdrop: true,
            close_on_escape: true,
            on_close: None,
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_panel: Cell::new((0.0, 0.0, 0.0, 0.0)),
        }
    }

    /// Share open state with an external `Rc<RefCell<bool>>` (e.g. from a trigger button).
    pub fn shared_open(mut self, flag: Rc<RefCell<bool>>) -> Self {
        self.open = flag;
        self
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn close_on_backdrop(mut self, v: bool) -> Self {
        self.close_on_backdrop = v;
        self
    }

    pub fn close_on_escape(mut self, v: bool) -> Self {
        self.close_on_escape = v;
        self
    }

    pub fn on_close(mut self, f: impl Fn() + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }

    pub fn is_open(&self) -> bool {
        *self.open.borrow()
    }

    fn close(&self) {
        *self.open.borrow_mut() = false;
        if let Some(f) = &self.on_close {
            f();
        }
    }

    fn panel_rect(&self, bounds: Rect, theme: &Theme) -> Rect {
        let title_h = if self.title.is_some() {
            TITLE_HEIGHT
        } else {
            0.0
        };
        let (_, ch) = self.content.intrinsic_size(theme);
        let inner_h = ch + PANEL_PADDING * 2.0;
        let h = (title_h + inner_h).min(bounds.height * 0.9).max(120.0);
        let w = self.width.min(bounds.width * 0.95);
        let x = bounds.x + (bounds.width - w) / 2.0;
        let y = bounds.y + (bounds.height - h) / 2.0;
        Rect::new(x, y, w, h)
    }

    fn content_rect(&self, panel: Rect) -> Rect {
        let title_h = if self.title.is_some() {
            TITLE_HEIGHT
        } else {
            0.0
        };
        Rect::new(
            panel.x + PANEL_PADDING,
            panel.y + title_h,
            panel.width - PANEL_PADDING * 2.0,
            panel.height - title_h - PANEL_PADDING,
        )
    }
}

impl Widget for Dialog {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_container(&self) -> bool {
        true
    }

    /// When open, Tab navigation and pointer focus stay inside this dialog.
    fn blocks_focus_outside(&self) -> bool {
        self.is_open()
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        self.last_bounds
            .set((bounds.x, bounds.y, bounds.width, bounds.height));

        if !self.is_open() {
            return;
        }

        // Backdrop
        renderer.fill_rect(bounds, Color::BLACK.with_alpha(0.55), Corners::ZERO);

        let panel = self.panel_rect(bounds, theme);
        self.last_panel
            .set((panel.x, panel.y, panel.width, panel.height));

        renderer.fill_rect(panel, theme.bg_elevated, Corners::all(theme.radius_lg));
        renderer.stroke_rect(panel, theme.border, 1.0, Corners::all(theme.radius_lg));

        if let Some(title) = &self.title {
            let opts = TextOptions {
                font_size: theme.font_size_lg,
                color: theme.fg,
                bold: true,
                ..Default::default()
            };
            renderer.draw_text(
                title,
                Point::new(panel.x + PANEL_PADDING, panel.y + 14.0),
                &opts,
            );
            // Divider under title
            renderer.fill_rect(
                Rect::new(
                    panel.x,
                    panel.y + TITLE_HEIGHT - 1.0,
                    panel.width,
                    1.0,
                ),
                theme.border,
                Corners::ZERO,
            );
        }

        let content_bounds = self.content_rect(panel);
        self.content.draw(renderer, content_bounds, theme);
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        if !self.is_open() {
            return EventStatus::Ignored;
        }

        let (bx, by, bw, bh) = self.last_bounds.get();
        let bounds = if bw > 0.0 {
            Rect::new(bx, by, bw, bh)
        } else {
            bounds
        };
        let panel = self.panel_rect(bounds, &Theme::dark());
        let content_bounds = self.content_rect(panel);

        match event {
            Event::KeyDown { key: Key::Escape, .. } if self.close_on_escape => {
                self.close();
                EventStatus::Consumed
            }
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if panel.contains(pos.x, pos.y) {
                    self.content.handle_event(event, content_bounds)
                } else if self.close_on_backdrop {
                    self.close();
                    EventStatus::Consumed
                } else {
                    // Block clicks from reaching widgets below.
                    EventStatus::Consumed
                }
            }
            Event::MouseUp {
                pos,
                button: MouseButton::Left,
            } => {
                if panel.contains(pos.x, pos.y) {
                    self.content.handle_event(event, content_bounds)
                } else {
                    EventStatus::Consumed
                }
            }
            Event::MouseMove { pos } => {
                if panel.contains(pos.x, pos.y) {
                    self.content.handle_event(event, content_bounds)
                } else {
                    EventStatus::Ignored
                }
            }
            Event::MouseClick { pos, .. } | Event::MouseDoubleClick { pos, .. } => {
                if panel.contains(pos.x, pos.y) {
                    self.content.handle_event(event, content_bounds)
                } else {
                    EventStatus::Consumed
                }
            }
            Event::Scroll { pos, .. } => {
                if panel.contains(pos.x, pos.y) {
                    self.content.handle_event(event, content_bounds)
                } else {
                    EventStatus::Consumed
                }
            }
            // Keyboard / text / IME — route to panel content.
            Event::KeyDown { .. }
            | Event::KeyUp { .. }
            | Event::TextInput { .. }
            | Event::ImePreedit { .. }
            | Event::FocusGained
            | Event::FocusLost => self.content.handle_event(event, content_bounds),
            _ => EventStatus::Consumed,
        }
    }

    fn layout_children<'a>(&'a self, _bounds: Rect, theme: &Theme) -> Vec<(&'a dyn Widget, Rect)> {
        if !self.is_open() {
            return vec![];
        }
        let (bx, by, bw, bh) = self.last_bounds.get();
        if bw <= 0.0 {
            return vec![];
        }
        let bounds = Rect::new(bx, by, bw, bh);
        let panel = self.panel_rect(bounds, theme);
        let content_bounds = self.content_rect(panel);
        vec![(self.content.as_ref() as &dyn Widget, content_bounds)]
    }

    fn layout_children_mut<'a>(
        &'a mut self,
        _bounds: Rect,
        theme: &Theme,
    ) -> Vec<(&'a mut dyn Widget, Rect)> {
        if !self.is_open() {
            return vec![];
        }
        let (bx, by, bw, bh) = self.last_bounds.get();
        if bw <= 0.0 {
            return vec![];
        }
        let bounds = Rect::new(bx, by, bw, bh);
        let panel = self.panel_rect(bounds, theme);
        let content_bounds = self.content_rect(panel);
        vec![(
            self.content.as_mut() as &mut dyn Widget,
            content_bounds,
        )]
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        // Overlay fills parent Stack.
        (0.0, 0.0)
    }
}

/// Shorthand constructor.
pub fn dialog(content: impl Widget + 'static) -> Dialog {
    Dialog::new(content)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("dialog-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
