//! Text input widget — full HTML-like editing behaviour.
//!
//! Supports: cursor movement, text selection, clipboard, IME, scroll, password mask.

use std::cell::Cell;

use crate::event::{Event, EventStatus, Key, MouseButton};
use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, CursorStyle, Theme};
use crate::widget::Widget;

// ── Clipboard helpers ─────────────────────────────────────────────────────────

fn clipboard_copy(text: &str) {
    if let Ok(mut cb) = arboard::Clipboard::new() {
        let _ = cb.set_text(text);
    }
}

fn clipboard_paste() -> Option<String> {
    arboard::Clipboard::new().ok()?.get_text().ok()
}

// ── InputSize ─────────────────────────────────────────────────────────────────

/// Height presets for Input (like shadcn-ui size variants).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl InputSize {
    pub fn height(&self, font_size: f32) -> f32 {
        match self {
            Self::Sm => font_size + 12.0,
            Self::Md => font_size + 20.0,
            Self::Lg => font_size + 28.0,
        }
    }
}

// ── Input struct ──────────────────────────────────────────────────────────────

pub struct Input {
    id:          String,
    value:       String,
    placeholder: String,
    preedit:     String,

    // Cursor & selection (byte indices into `value`)
    cursor_byte: usize,
    sel_anchor:  Option<usize>,

    // Horizontal scroll (pixel offset, updated lazily during draw)
    scroll_x:    Cell<f32>,

    focused:   bool,
    hovered:   bool,
    disabled:  bool,
    clearable: bool,
    password:  bool,
    size:      InputSize,
    width:     Option<f32>,

    on_change: Option<Box<dyn Fn(&str)>>,
    on_submit: Option<Box<dyn Fn(&str)>>,
    on_clear:  Option<Box<dyn Fn()>>,
}

// ── Constructor / builders ────────────────────────────────────────────────────

impl Input {
    pub fn new(value: impl Into<String>) -> Self {
        let v: String = value.into();
        let end = v.len();
        Self {
            id:          uuid(),
            value:       v,
            placeholder: String::new(),
            preedit:     String::new(),
            cursor_byte: end,
            sel_anchor:  None,
            scroll_x:    Cell::new(0.0),
            focused:   false,
            hovered:   false,
            disabled:  false,
            clearable: false,
            password:  false,
            size:      InputSize::Md,
            width:     None,
            on_change: None,
            on_submit: None,
            on_clear:  None,
        }
    }

    pub fn placeholder(mut self, p: impl Into<String>) -> Self { self.placeholder = p.into(); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    /// Fixed pixel width. Without this, the input fills its parent container.
    pub fn width(mut self, w: f32) -> Self { self.width = Some(w); self }
    /// Height preset: Sm / Md (default) / Lg.
    pub fn size(mut self, s: InputSize) -> Self { self.size = s; self }
    pub fn clearable(mut self) -> Self { self.clearable = true; self }
    pub fn password(mut self) -> Self { self.password = true; self }
    pub fn on_change(mut self, f: impl Fn(&str) + 'static) -> Self { self.on_change = Some(Box::new(f)); self }
    pub fn on_submit(mut self, f: impl Fn(&str) + 'static) -> Self { self.on_submit = Some(Box::new(f)); self }
    pub fn on_clear(mut self,  f: impl Fn()   + 'static) -> Self { self.on_clear  = Some(Box::new(f)); self }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

impl Input {
    fn is_interactive(&self) -> bool { !self.disabled }

    fn draw_width(&self, bounds_w: f32) -> f32 {
        self.width.map(|w| w.min(bounds_w)).unwrap_or(bounds_w)
    }

    fn clear_btn_rect(&self, db: Rect) -> Rect {
        let sz = 16.0;
        Rect::new(db.x + db.width - sz - 8.0, db.y + (db.height - sz) / 2.0, sz, sz)
    }

    // ── Cursor movement ───────────────────────────────────────────────────────

    /// Byte index of the char immediately before the cursor.
    fn byte_left(&self) -> usize {
        if self.cursor_byte == 0 { return 0; }
        self.value[..self.cursor_byte]
            .char_indices()
            .next_back()
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Byte index right after the char at (or following) the cursor.
    fn byte_right(&self) -> usize {
        self.value[self.cursor_byte..]
            .chars()
            .next()
            .map(|c| self.cursor_byte + c.len_utf8())
            .unwrap_or(self.value.len())
    }

    // ── Selection ─────────────────────────────────────────────────────────────

    fn selection_range(&self) -> Option<(usize, usize)> {
        self.sel_anchor.map(|a| {
            if a <= self.cursor_byte { (a, self.cursor_byte) }
            else                     { (self.cursor_byte, a) }
        })
    }

    fn has_selection(&self) -> bool {
        self.sel_anchor.map(|a| a != self.cursor_byte).unwrap_or(false)
    }

    /// Delete the selected region. Returns true if something was deleted.
    fn delete_selection(&mut self) -> bool {
        if let Some((s, e)) = self.selection_range() {
            if s < e {
                self.value.drain(s..e);
                self.cursor_byte = s;
                self.sel_anchor  = None;
                return true;
            }
        }
        self.sel_anchor = None;
        false
    }

    /// Extend/start selection: call before moving cursor.
    fn start_or_extend_sel(&mut self) {
        if self.sel_anchor.is_none() {
            self.sel_anchor = Some(self.cursor_byte);
        }
    }

    // ── Pixel layout helpers ──────────────────────────────────────────────────

    /// Approximate pixel width of `value[..byte]` in the current display mode.
    fn pixel_x_at(&self, byte: usize, font_size: f32) -> f32 {
        if self.password {
            let nchars = self.value[..byte].chars().count();
            nchars as f32 * font_size * 0.85
        } else {
            self.value[..byte].chars().map(|c| char_px(c, font_size)).sum()
        }
    }

    /// Find the byte offset closest to a given pixel x (for click-to-cursor).
    fn byte_at_pixel(&self, x: f32, font_size: f32) -> usize {
        let mut acc = 0.0_f32;
        for (byte_pos, c) in self.value.char_indices() {
            let cw = if self.password { font_size * 0.85 } else { char_px(c, font_size) };
            if acc + cw / 2.0 >= x { return byte_pos; }
            acc += cw;
        }
        self.value.len()
    }
}

/// Approximate display width of a single character.
fn char_px(c: char, font_size: f32) -> f32 {
    if c.is_ascii() { font_size * 0.6 } else { font_size }
}

// ── Widget impl ───────────────────────────────────────────────────────────────

impl Widget for Input {
    fn id(&self) -> &str { &self.id }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let draw_w  = self.draw_width(bounds.width);
        let input_h = self.size.height(theme.font_size_md);
        let db = Rect::new(bounds.x, bounds.y, draw_w, input_h);

        // ── Border / background ───────────────────────────────────────────────
        let border_color = if self.disabled        { theme.border.with_alpha(0.5) }
                           else if self.focused     { theme.accent }
                           else if self.hovered     { theme.fg_subtle }
                           else                     { theme.border };
        let radius = Corners::all(theme.radius_md);
        renderer.fill_rect(db, theme.bg_elevated, radius);
        renderer.stroke_rect(db, border_color, 1.5, radius);

        let show_clear  = self.clearable && !self.value.is_empty() && !self.disabled;
        let right_pad   = if show_clear { 32.0 } else { 10.0 };
        let left_pad    = 10.0;
        let text_area_w = (db.width - left_pad - right_pad).max(0.0);
        let px          = db.x + left_pad;
        let py          = db.y + (input_h - theme.font_size_md) / 2.0;
        let fs          = theme.font_size_md;

        // ── Update scroll so cursor stays visible ─────────────────────────────
        let cursor_px = self.pixel_x_at(self.cursor_byte, fs);
        let mut sx    = self.scroll_x.get();
        if cursor_px < sx + 2.0 {
            sx = (cursor_px - 2.0).max(0.0);
        } else if cursor_px > sx + text_area_w - 4.0 {
            sx = cursor_px - text_area_w + 4.0;
        }
        self.scroll_x.set(sx);

        // ── Clip to text area (prevents overflow rendering) ───────────────────
        let clip = Rect::new(db.x + left_pad - 1.0, db.y + 2.0,
                             text_area_w + 1.0, input_h - 4.0);
        renderer.push_clip(clip);

        let opts = TextOptions {
            font_size: fs,
            max_width: None,
            ..Default::default()
        };

        let text_origin_x = px - sx;

        if self.value.is_empty() && self.preedit.is_empty() {
            // ── Placeholder ───────────────────────────────────────────────────
            let ph_color = if self.disabled { theme.fg_subtle.with_alpha(0.5) }
                           else             { theme.fg_subtle };
            renderer.draw_text(&self.placeholder, Point::new(text_origin_x, py),
                               &TextOptions { color: ph_color, ..opts.clone() });
        } else {
            // ── Selection highlight ───────────────────────────────────────────
            if self.focused {
                if let Some((s, e)) = self.selection_range() {
                    if s < e {
                        let sx_px = text_origin_x + self.pixel_x_at(s, fs);
                        let ex_px = text_origin_x + self.pixel_x_at(e, fs);
                        let sel_rect = Rect::new(
                            sx_px.max(db.x + left_pad - 1.0),
                            db.y + 2.0,
                            (ex_px - sx_px).max(0.0),
                            input_h - 4.0,
                        );
                        renderer.fill_rect(sel_rect, theme.accent.with_alpha(0.25), Corners::ZERO);
                    }
                }
            }

            // ── Value text ────────────────────────────────────────────────────
            let val_color = if self.disabled { theme.fg.with_alpha(0.5) } else { theme.fg };
            let val_str: String;
            let display_val = if self.password {
                val_str = "\u{25cf}".repeat(self.value.chars().count());
                val_str.as_str()
            } else {
                self.value.as_str()
            };
            let (vw, _) = renderer.draw_text(display_val, Point::new(text_origin_x, py),
                                             &TextOptions { color: val_color, ..opts.clone() });

            // ── IME preedit ───────────────────────────────────────────────────
            if !self.preedit.is_empty() {
                let pre_x = text_origin_x + vw;
                let (pw, _) = renderer.draw_text(
                    &self.preedit, Point::new(pre_x, py),
                    &TextOptions { color: theme.fg_subtle, ..opts },
                );
                renderer.draw_line(
                    Point::new(pre_x,      py + fs + 1.0),
                    Point::new(pre_x + pw, py + fs + 1.0),
                    theme.fg_subtle, 1.0,
                );
            }
        }

        // ── Text cursor (only when no selection) ──────────────────────────────
        if self.focused && !self.has_selection() && self.preedit.is_empty() {
            let cx = (px + cursor_px - sx)
                .max(px)
                .min(px + text_area_w);
            renderer.draw_line(
                Point::new(cx, py + 1.0),
                Point::new(cx, py + fs - 1.0),
                theme.fg, 1.5,
            );
        }

        renderer.pop_clip();

        // ── Clear button × ────────────────────────────────────────────────────
        if show_clear {
            let cr        = self.clear_btn_rect(db);
            let icon_col  = if self.hovered { theme.fg } else { theme.fg_subtle };
            let (cx, cy)  = (cr.x + cr.width / 2.0, cr.y + cr.height / 2.0);
            let arm       = 4.0;
            renderer.draw_line(Point::new(cx-arm, cy-arm), Point::new(cx+arm, cy+arm), icon_col, 1.5);
            renderer.draw_line(Point::new(cx+arm, cy-arm), Point::new(cx-arm, cy+arm), icon_col, 1.5);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> EventStatus {
        let draw_w  = self.draw_width(bounds.width);
        let input_h = self.size.height(Theme::default().font_size_md);
        let db      = Rect::new(bounds.x, bounds.y, draw_w, input_h);
        let fs      = Theme::default().font_size_md;

        match event {
            // ── Mouse ─────────────────────────────────────────────────────────
            Event::MouseMove { pos } => {
                self.hovered = db.contains(pos.x, pos.y);
                EventStatus::Ignored
            }

            Event::MouseDown { pos, button: MouseButton::Left } => {
                if db.contains(pos.x, pos.y) && self.is_interactive() {
                    // Clear button hit test
                    if self.clearable && !self.value.is_empty() {
                        if self.clear_btn_rect(db).contains(pos.x, pos.y) {
                            self.value.clear();
                            self.preedit.clear();
                            self.cursor_byte = 0;
                            self.sel_anchor  = None;
                            self.scroll_x.set(0.0);
                            if let Some(f) = &self.on_change { f(&self.value); }
                            if let Some(f) = &self.on_clear  { f(); }
                            return EventStatus::Consumed;
                        }
                    }
                    // Click to position cursor
                    let left_pad = 10.0;
                    let click_x  = pos.x - db.x - left_pad + self.scroll_x.get();
                    self.cursor_byte = self.byte_at_pixel(click_x.max(0.0), fs);
                    self.sel_anchor  = None;
                    self.focused     = true;
                    EventStatus::Consumed
                } else {
                    self.focused    = false;
                    self.sel_anchor = None;
                    EventStatus::Ignored
                }
            }

            // ── Text input ────────────────────────────────────────────────────
            Event::TextInput { text } if self.focused => {
                self.preedit.clear();
                self.delete_selection();
                let at = self.cursor_byte;
                self.value.insert_str(at, text);
                self.cursor_byte += text.len();
                if let Some(f) = &self.on_change { f(&self.value); }
                EventStatus::Consumed
            }

            // ── IME preedit ───────────────────────────────────────────────────
            Event::ImePreedit { text, .. } if self.focused => {
                self.preedit = text.clone();
                EventStatus::Consumed
            }

            // ── Keyboard ─────────────────────────────────────────────────────
            Event::KeyDown { key, modifiers } if self.focused => {
                let ctrl  = modifiers.ctrl || modifiers.meta;
                let shift = modifiers.shift;

                match key {
                    // ── Arrow Left ────────────────────────────────────────────
                    Key::ArrowLeft => {
                        if shift {
                            self.start_or_extend_sel();
                            self.cursor_byte = self.byte_left();
                        } else if self.has_selection() {
                            let (s, _) = self.selection_range().unwrap();
                            self.cursor_byte = s;
                            self.sel_anchor  = None;
                        } else {
                            self.cursor_byte = self.byte_left();
                        }
                    }

                    // ── Arrow Right ───────────────────────────────────────────
                    Key::ArrowRight => {
                        if shift {
                            self.start_or_extend_sel();
                            self.cursor_byte = self.byte_right();
                        } else if self.has_selection() {
                            let (_, e) = self.selection_range().unwrap();
                            self.cursor_byte = e;
                            self.sel_anchor  = None;
                        } else {
                            self.cursor_byte = self.byte_right();
                        }
                    }

                    // ── Home ──────────────────────────────────────────────────
                    Key::Home => {
                        if shift { self.start_or_extend_sel(); } else { self.sel_anchor = None; }
                        self.cursor_byte = 0;
                    }

                    // ── End ───────────────────────────────────────────────────
                    Key::End => {
                        if shift { self.start_or_extend_sel(); } else { self.sel_anchor = None; }
                        self.cursor_byte = self.value.len();
                    }

                    // ── Backspace ─────────────────────────────────────────────
                    Key::Backspace => {
                        if !self.preedit.is_empty() {
                            // Remove last char from preedit
                            let new_len = self.preedit
                                .char_indices().next_back().map(|(i,_)| i).unwrap_or(0);
                            self.preedit.truncate(new_len);
                        } else if self.delete_selection() {
                            if let Some(f) = &self.on_change { f(&self.value); }
                        } else if self.cursor_byte > 0 {
                            let new_pos = self.byte_left();
                            self.value.drain(new_pos..self.cursor_byte);
                            self.cursor_byte = new_pos;
                            if let Some(f) = &self.on_change { f(&self.value); }
                        }
                    }

                    // ── Delete ────────────────────────────────────────────────
                    Key::Delete => {
                        if self.delete_selection() {
                            if let Some(f) = &self.on_change { f(&self.value); }
                        } else if self.cursor_byte < self.value.len() {
                            let next = self.byte_right();
                            self.value.drain(self.cursor_byte..next);
                            if let Some(f) = &self.on_change { f(&self.value); }
                        }
                    }

                    // ── Enter ─────────────────────────────────────────────────
                    Key::Enter => {
                        self.preedit.clear();
                        if let Some(f) = &self.on_submit { f(&self.value); }
                    }

                    // ── Escape ────────────────────────────────────────────────
                    Key::Escape => {
                        if !self.preedit.is_empty() {
                            self.preedit.clear();
                        } else {
                            self.focused    = false;
                            self.sel_anchor = None;
                        }
                    }

                    // ── Ctrl shortcuts ────────────────────────────────────────
                    Key::Char('a') if ctrl => {
                        self.sel_anchor  = Some(0);
                        self.cursor_byte = self.value.len();
                    }
                    Key::Char('c') if ctrl => {
                        if let Some((s, e)) = self.selection_range() {
                            if s < e { clipboard_copy(&self.value[s..e]); }
                        }
                    }
                    Key::Char('x') if ctrl => {
                        if let Some((s, e)) = self.selection_range() {
                            if s < e {
                                clipboard_copy(&self.value[s..e]);
                                self.delete_selection();
                                if let Some(f) = &self.on_change { f(&self.value); }
                            }
                        }
                    }
                    Key::Char('v') if ctrl => {
                        if let Some(text) = clipboard_paste() {
                            self.delete_selection();
                            let at = self.cursor_byte;
                            self.value.insert_str(at, &text);
                            self.cursor_byte += text.len();
                            if let Some(f) = &self.on_change { f(&self.value); }
                        }
                    }

                    _ => {}
                }
                EventStatus::Consumed
            }

            Event::FocusLost => {
                self.focused    = false;
                self.sel_anchor = None;
                self.preedit.clear();
                EventStatus::Ignored
            }

            _ => EventStatus::Ignored,
        }
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        (self.width.unwrap_or(240.0), self.size.height(theme.font_size_md))
    }

    fn cursor_at(&self, pos: (f32, f32), bounds: Rect) -> CursorStyle {
        let draw_w  = self.draw_width(bounds.width);
        let input_h = self.size.height(Theme::default().font_size_md);
        let db = Rect::new(bounds.x, bounds.y, draw_w, input_h);
        if !db.contains(pos.0, pos.1) { return CursorStyle::Default; }
        if self.disabled { CursorStyle::NotAllowed } else { CursorStyle::Text }
    }
}

// ── Public constructors ───────────────────────────────────────────────────────

pub fn input(value: impl Into<String>) -> Input { Input::new(value) }

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("input-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
