//! Toast / Sonner — stacked notifications (shadcn/ui inspired).
//!
//! Mount a [`Toaster`] at the app root (full bounds) and push messages via
//! [`ToastStore`]:
//!
//! ```rust,ignore
//! let store = toast_store();
//!
//! stack![
//!     main_ui,
//!     toaster(store.clone()),
//! ];
//!
//! button("Save").on_click({
//!     let store = store.clone();
//!     move || {
//!         store.borrow_mut().success("Saved");
//!     }
//! });
//! ```

use std::cell::Cell;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};

use crate::color::Color;
use crate::event::{Event, EventStatus, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

const TOAST_W: f32 = 356.0;
const TOAST_GAP: f32 = 8.0;
const EDGE_MARGIN: f32 = 16.0;
const PAD_X: f32 = 16.0;
const PAD_Y: f32 = 14.0;
const CLOSE_SIZE: f32 = 20.0;
const DEFAULT_DURATION_MS: u64 = 4000;

/// Visual style for a toast notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    #[default]
    Default,
    Success,
    Error,
    Warning,
}

/// Corner placement for the toast stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    #[default]
    BottomRight,
    BottomLeft,
    TopRight,
    TopLeft,
}

/// A single toast message.
#[derive(Debug, Clone)]
pub struct ToastItem {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    /// Auto-dismiss after this many milliseconds. `0` = persist until closed.
    pub duration_ms: u64,
    pub created: Instant,
}

impl ToastItem {
    pub fn new(title: impl Into<String>) -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            title: title.into(),
            description: None,
            variant: ToastVariant::Default,
            duration_ms: DEFAULT_DURATION_MS,
            created: Instant::now(),
        }
    }

    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn duration_ms(mut self, ms: u64) -> Self {
        self.duration_ms = ms;
        self
    }

    fn is_expired(&self) -> bool {
        self.duration_ms > 0 && self.created.elapsed() >= Duration::from_millis(self.duration_ms)
    }

    fn accent_color(&self, theme: &Theme) -> Color {
        match self.variant {
            ToastVariant::Default => theme.border,
            ToastVariant::Success => Color::hex("#22c55e"),
            ToastVariant::Error => theme.danger,
            ToastVariant::Warning => Color::hex("#f59e0b"),
        }
    }

    fn measure_height(&self, width: f32, theme: &Theme) -> f32 {
        let text_w = width - PAD_X * 2.0 - CLOSE_SIZE - 8.0;
        let title_opts = TextOptions {
            font_size: theme.font_size_md,
            color: theme.fg,
            bold: true,
            max_width: Some(text_w),
            ..Default::default()
        };
        let (_, title_h, _) = measure_lines(&self.title, &title_opts, text_w);

        let desc_h = if let Some(desc) = &self.description {
            let desc_opts = TextOptions {
                font_size: theme.font_size_sm,
                color: theme.fg_muted,
                max_width: Some(text_w),
                ..Default::default()
            };
            let (_, h, _) = measure_lines(desc, &desc_opts, text_w);
            h + 4.0
        } else {
            0.0
        };

        (PAD_Y * 2.0 + title_h + desc_h).max(52.0)
    }
}

/// Shared toast queue — clone the `Rc` into callbacks.
#[derive(Default)]
pub struct ToastStore {
    toasts: Vec<ToastItem>,
}

impl ToastStore {
    pub fn push(&mut self, item: ToastItem) -> u64 {
        let id = item.id;
        self.toasts.push(item);
        id
    }

    pub fn show(&mut self, title: impl Into<String>) -> u64 {
        self.push(ToastItem::new(title))
    }

    pub fn message(&mut self, title: impl Into<String>, description: impl Into<String>) -> u64 {
        self.push(ToastItem::new(title).description(description))
    }

    pub fn success(&mut self, title: impl Into<String>) -> u64 {
        self.push(ToastItem::new(title).variant(ToastVariant::Success))
    }

    pub fn error(&mut self, title: impl Into<String>) -> u64 {
        self.push(ToastItem::new(title).variant(ToastVariant::Error))
    }

    pub fn warning(&mut self, title: impl Into<String>) -> u64 {
        self.push(ToastItem::new(title).variant(ToastVariant::Warning))
    }

    pub fn dismiss(&mut self, id: u64) {
        self.toasts.retain(|t| t.id != id);
    }

    pub fn dismiss_all(&mut self) {
        self.toasts.clear();
    }

    pub fn purge_expired(&mut self) {
        self.toasts.retain(|t| !t.is_expired());
    }

    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    fn needs_timer(&self) -> bool {
        self.toasts.iter().any(|t| t.duration_ms > 0 && !t.is_expired())
    }
}

/// Create a shared toast store for use with [`toaster`].
pub fn toast_store() -> Rc<RefCell<ToastStore>> {
    Rc::new(RefCell::new(ToastStore::default()))
}

/// Renders and manages toasts. Use `(0, 0)` intrinsic size inside a [`Stack`](crate::widget::stack::Stack)
/// or mount on the app root with full window bounds.
pub struct Toaster {
    id: String,
    store: Rc<RefCell<ToastStore>>,
    position: ToastPosition,
    last_bounds: Cell<(f32, f32, f32, f32)>,
    last_hit_regions: RefCell<Vec<(u64, (f32, f32, f32, f32), (f32, f32, f32, f32))>>,
}

impl Toaster {
    pub fn new(store: Rc<RefCell<ToastStore>>) -> Self {
        Self {
            id: uuid(),
            store,
            position: ToastPosition::BottomRight,
            last_bounds: Cell::new((0.0, 0.0, 0.0, 0.0)),
            last_hit_regions: RefCell::new(Vec::new()),
        }
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    fn toast_x(&self, bounds: Rect) -> f32 {
        match self.position {
            ToastPosition::BottomRight | ToastPosition::TopRight => {
                bounds.x + bounds.width - EDGE_MARGIN - TOAST_W
            }
            ToastPosition::BottomLeft | ToastPosition::TopLeft => bounds.x + EDGE_MARGIN,
        }
    }

    fn layout_toasts(&self, bounds: Rect, theme: &Theme) -> Vec<(ToastItem, Rect, Rect)> {
        let store = self.store.borrow();
        let x = self.toast_x(bounds);
        let mut regions = Vec::new();

        let mut offset = EDGE_MARGIN;
        for item in store.toasts.iter().rev() {
            let h = item.measure_height(TOAST_W, theme);
            let y = match self.position {
                ToastPosition::BottomRight | ToastPosition::BottomLeft => {
                    let y = bounds.y + bounds.height - offset - h;
                    offset += h + TOAST_GAP;
                    y
                }
                ToastPosition::TopRight | ToastPosition::TopLeft => {
                    let y = bounds.y + offset;
                    offset += h + TOAST_GAP;
                    y
                }
            };

            let toast_rect = Rect::new(x, y, TOAST_W, h);
            let close = Rect::new(
                toast_rect.x + toast_rect.width - PAD_X - CLOSE_SIZE,
                toast_rect.y + PAD_Y,
                CLOSE_SIZE,
                CLOSE_SIZE,
            );
            regions.push((item.clone(), toast_rect, close));
        }
        regions
    }

    fn draw_toast(
        &self,
        renderer: &mut dyn Renderer,
        item: &ToastItem,
        rect: Rect,
        theme: &Theme,
    ) {
        renderer.fill_rect(
            Rect::new(rect.x + 1.0, rect.y + 2.0, rect.width, rect.height),
            Color::BLACK.with_alpha(0.22),
            Corners::all(theme.radius_lg),
        );
        renderer.fill_rect(rect, theme.bg_surface, Corners::all(theme.radius_lg));
        renderer.stroke_rect(rect, theme.border, 1.0, Corners::all(theme.radius_lg));

        // Left accent bar (variant indicator)
        renderer.fill_rect(
            Rect::new(rect.x, rect.y + 8.0, 3.0, rect.height - 16.0),
            item.accent_color(theme),
            Corners::all(1.5),
        );

        let text_w = rect.width - PAD_X * 2.0 - CLOSE_SIZE - 8.0;
        let title_opts = TextOptions {
            font_size: theme.font_size_md,
            color: theme.fg,
            bold: true,
            max_width: Some(text_w),
            ..Default::default()
        };
        let (_, title_h, _) = renderer.measure_text(&item.title, &title_opts);
        renderer.draw_text(
            &item.title,
            Point::new(rect.x + PAD_X + 4.0, rect.y + PAD_Y),
            &title_opts,
        );

        if let Some(desc) = &item.description {
            let desc_opts = TextOptions {
                font_size: theme.font_size_sm,
                color: theme.fg_muted,
                max_width: Some(text_w),
                ..Default::default()
            };
            renderer.draw_text(
                desc,
                Point::new(rect.x + PAD_X + 4.0, rect.y + PAD_Y + title_h + 4.0),
                &desc_opts,
            );
        }

        // Close button (×)
        let close = Rect::new(
            rect.x + rect.width - PAD_X - CLOSE_SIZE,
            rect.y + PAD_Y,
            CLOSE_SIZE,
            CLOSE_SIZE,
        );
        renderer.fill_rect(close, theme.bg_elevated.with_alpha(0.6), Corners::all(theme.radius_sm));
        let x_color = theme.fg_muted;
        let cx = close.x + close.width / 2.0;
        let cy = close.y + close.height / 2.0;
        renderer.draw_line(
            Point::new(cx - 4.0, cy - 4.0),
            Point::new(cx + 4.0, cy + 4.0),
            x_color,
            1.5,
        );
        renderer.draw_line(
            Point::new(cx + 4.0, cy - 4.0),
            Point::new(cx - 4.0, cy + 4.0),
            x_color,
            1.5,
        );
    }

    fn region_at(&self, pos: Point) -> Option<(u64, bool)> {
        for (id, toast, close) in self.last_hit_regions.borrow().iter().rev() {
            let tr = Rect::new(toast.0, toast.1, toast.2, toast.3);
            if !tr.contains(pos.x, pos.y) {
                continue;
            }
            let cr = Rect::new(close.0, close.1, close.2, close.3);
            return Some((*id, cr.contains(pos.x, pos.y)));
        }
        None
    }
}

impl Widget for Toaster {
    fn id(&self) -> &str {
        &self.id
    }

    fn wants_redraw(&self) -> bool {
        let mut store = self.store.borrow_mut();
        store.purge_expired();
        store.needs_timer()
    }

    fn draw(&self, _renderer: &mut dyn Renderer, _bounds: Rect, _theme: &Theme) {}

    fn draw_overlay(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        self.last_bounds
            .set((bounds.x, bounds.y, bounds.width, bounds.height));

        let mut store = self.store.borrow_mut();
        store.purge_expired();
        if store.is_empty() {
            self.last_hit_regions.borrow_mut().clear();
            return;
        }

        let layout = self.layout_toasts(bounds, theme);
        let mut hits = Vec::new();
        for (item, toast_rect, close_rect) in &layout {
            self.draw_toast(renderer, item, *toast_rect, theme);
            hits.push((
                item.id,
                (
                    toast_rect.x,
                    toast_rect.y,
                    toast_rect.width,
                    toast_rect.height,
                ),
                (close_rect.x, close_rect.y, close_rect.width, close_rect.height),
            ));
        }
        *self.last_hit_regions.borrow_mut() = hits;
    }

    fn hit_test(&self, pos: (f32, f32), _bounds: Rect, _theme: &Theme) -> bool {
        self.region_at(Point::new(pos.0, pos.1)).is_some()
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let theme = Theme::default();
        let layout = self.layout_toasts(bounds, &theme);
        *self.last_hit_regions.borrow_mut() = layout
                .iter()
                .map(|(item, tr, cr)| {
                    (
                        item.id,
                        (tr.x, tr.y, tr.width, tr.height),
                        (cr.x, cr.y, cr.width, cr.height),
                    )
                })
                .collect();

        match event {
            Event::MouseDown {
                pos,
                button: MouseButton::Left,
            } => {
                if let Some((id, on_close)) = self.region_at(*pos) {
                    if on_close {
                        self.store.borrow_mut().dismiss(id);
                    }
                    return EventStatus::Consumed;
                }
                EventStatus::Ignored
            }
            Event::MouseMove { pos } => {
                if self.region_at(*pos).is_some() {
                    return EventStatus::Ignored;
                }
                EventStatus::Ignored
            }
            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn cursor_at(&self, pos: (f32, f32), _bounds: Rect) -> CursorStyle {
        if let Some((_, on_close)) = self.region_at(Point::new(pos.0, pos.1)) {
            if on_close {
                return CursorStyle::Pointer;
            }
        }
        CursorStyle::Default
    }
}

/// Shorthand constructor.
pub fn toaster(store: Rc<RefCell<ToastStore>>) -> Toaster {
    Toaster::new(store)
}

fn measure_lines(text: &str, opts: &TextOptions, max_w: f32) -> (f32, f32, f32) {
    // Rough line-wrap estimate when no renderer is available.
    let char_w = opts.font_size * 0.55;
    let chars_per_line = (max_w / char_w).max(1.0) as usize;
    let lines = text.chars().count().div_ceil(chars_per_line).max(1) as f32;
    let line_h = opts.font_size * 1.35;
    (max_w, lines * line_h, line_h)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("toaster-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
