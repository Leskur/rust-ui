//! Input event types passed from the platform layer to widgets.

use crate::render::Point;

/// Mouse button identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// A keyboard key (simplified — expand as needed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Enter,
    Backspace,
    Delete,
    Tab,
    Escape,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
}

/// Keyboard modifier state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub ctrl:  bool,
    pub shift: bool,
    pub alt:   bool,
    pub meta:  bool,
}

/// All input events that the runtime delivers to the widget tree.
#[derive(Debug, Clone)]
pub enum Event {
    // ── Mouse ────────────────────────────────────────────────────────────────
    MouseMove { pos: Point },
    MouseDown { pos: Point, button: MouseButton },
    MouseUp   { pos: Point, button: MouseButton },
    MouseClick { pos: Point, button: MouseButton },
    MouseDoubleClick { pos: Point, button: MouseButton },
    Scroll    { pos: Point, delta_x: f32, delta_y: f32 },

    // ── Keyboard ─────────────────────────────────────────────────────────────
    KeyDown { key: Key, modifiers: Modifiers },
    KeyUp   { key: Key, modifiers: Modifiers },
    TextInput { text: String },
    /// IME preedit text changed. Empty text means preedit was cancelled.
    ImePreedit { text: String, cursor: Option<(usize, usize)> },

    // ── Focus ────────────────────────────────────────────────────────────────
    FocusGained,
    FocusLost,

    // ── Window ───────────────────────────────────────────────────────────────
    Resized { width: f32, height: f32 },
    CloseRequested,
}

/// How a widget responds after handling an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    /// This widget consumed the event — don't propagate further.
    Consumed,
    /// This widget ignored the event — propagate to next handler.
    Ignored,
}
