//! Style system — composable, CSS-inspired styling for widgets.
//!
//! # Philosophy
//! Instead of per-widget style closures, rust-ui uses a single `Style` value
//! that can be created once, stored, cloned, and merged — similar to CSS classes.
//!
//! ```rust
//! use rust_ui::style::Style;
//! use rust_ui::color::Color;
//!
//! let primary = Style::new()
//!     .bg(Color::hex("#5c7cfa"))
//!     .text_color(Color::WHITE)
//!     .radius(8.0)
//!     .padding_xy(16.0, 8.0);
//!
//! // Derive a variant by cloning and overriding
//! let danger = primary.clone().bg(Color::hex("#ff4d4f"));
//! ```

use crate::color::Color;

/// Four-sided spacing (top, right, bottom, left) — same as CSS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub const ZERO: Self = Self {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    pub fn all(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }
    pub fn xy(x: f32, y: f32) -> Self {
        Self {
            top: y,
            right: x,
            bottom: y,
            left: x,
        }
    }
    pub fn trbl(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl Default for Edges {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f32> for Edges {
    fn from(v: f32) -> Self {
        Self::all(v)
    }
}

impl From<(f32, f32)> for Edges {
    fn from((x, y): (f32, f32)) -> Self {
        Self::xy(x, y)
    }
}

/// Four-corner border radius.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Corners {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Corners {
    pub const ZERO: Self = Self {
        top_left: 0.0,
        top_right: 0.0,
        bottom_right: 0.0,
        bottom_left: 0.0,
    };

    pub fn all(v: f32) -> Self {
        Self {
            top_left: v,
            top_right: v,
            bottom_right: v,
            bottom_left: v,
        }
    }
}

impl Default for Corners {
    fn default() -> Self {
        Self::ZERO
    }
}

impl From<f32> for Corners {
    fn from(v: f32) -> Self {
        Self::all(v)
    }
}

/// Border definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: Corners,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Corners::ZERO,
        }
    }
}

/// Font weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin,
    Light,
    Regular,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Regular
    }
}

/// Text alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Start,
    Center,
    End,
}

/// A composable style value.
///
/// All fields are `Option` so styles can be merged: a `None` field means
/// "inherit from parent or use default".
#[derive(Debug, Clone, Default)]
pub struct Style {
    pub background: Option<Color>,
    pub text_color: Option<Color>,
    pub font_size: Option<f32>,
    pub font_weight: Option<FontWeight>,
    pub font_family: Option<String>,
    pub text_align: Option<TextAlign>,
    pub padding: Option<Edges>,
    pub margin: Option<Edges>,
    pub border: Option<Border>,
    pub width: Option<Size>,
    pub height: Option<Size>,
    pub opacity: Option<f32>,
    pub cursor: Option<CursorStyle>,
}

/// Dimension sizing — mirrors CSS sizing keywords.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    /// Fixed pixel value.
    Px(f32),
    /// Fill available space.
    Fill,
    /// Shrink to content.
    Shrink,
    /// Percentage of parent.
    Percent(f32),
}

impl Default for Size {
    fn default() -> Self {
        Self::Shrink
    }
}

/// Mouse cursor style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorStyle {
    #[default]
    Default,
    Pointer,
    Text,
    NotAllowed,
    Grab,
    Crosshair,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    // ── Background ──────────────────────────────────────────────────────────

    pub fn bg(mut self, color: impl Into<Color>) -> Self {
        self.background = Some(color.into());
        self
    }

    // ── Text ────────────────────────────────────────────────────────────────

    pub fn text_color(mut self, color: impl Into<Color>) -> Self {
        self.text_color = Some(color.into());
        self
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = Some(size);
        self
    }

    pub fn bold(mut self) -> Self {
        self.font_weight = Some(FontWeight::Bold);
        self
    }

    pub fn font_weight(mut self, w: FontWeight) -> Self {
        self.font_weight = Some(w);
        self
    }

    pub fn font_family(mut self, name: impl Into<String>) -> Self {
        self.font_family = Some(name.into());
        self
    }

    pub fn text_align(mut self, align: TextAlign) -> Self {
        self.text_align = Some(align);
        self
    }

    // ── Spacing ─────────────────────────────────────────────────────────────

    pub fn padding(mut self, edges: impl Into<Edges>) -> Self {
        self.padding = Some(edges.into());
        self
    }

    pub fn padding_xy(mut self, x: f32, y: f32) -> Self {
        self.padding = Some(Edges::xy(x, y));
        self
    }

    pub fn margin(mut self, edges: impl Into<Edges>) -> Self {
        self.margin = Some(edges.into());
        self
    }

    // ── Border / Radius ─────────────────────────────────────────────────────

    pub fn radius(mut self, r: impl Into<Corners>) -> Self {
        let r = r.into();
        let b = self.border.get_or_insert_with(Border::default);
        b.radius = r;
        self
    }

    pub fn border_color(mut self, color: impl Into<Color>) -> Self {
        let b = self.border.get_or_insert_with(Border::default);
        b.color = color.into();
        self
    }

    pub fn border_width(mut self, width: f32) -> Self {
        let b = self.border.get_or_insert_with(Border::default);
        b.width = width;
        self
    }

    // ── Size ────────────────────────────────────────────────────────────────

    pub fn width(mut self, w: impl Into<Size>) -> Self {
        self.width = Some(w.into());
        self
    }

    pub fn height(mut self, h: impl Into<Size>) -> Self {
        self.height = Some(h.into());
        self
    }

    pub fn fill_width(self) -> Self {
        self.width(Size::Fill)
    }
    pub fn fill_height(self) -> Self {
        self.height(Size::Fill)
    }

    // ── Misc ────────────────────────────────────────────────────────────────

    pub fn opacity(mut self, v: f32) -> Self {
        self.opacity = Some(v.clamp(0.0, 1.0));
        self
    }

    pub fn cursor(mut self, c: CursorStyle) -> Self {
        self.cursor = Some(c);
        self
    }

    // ── Merge ───────────────────────────────────────────────────────────────

    /// Merge `other` on top of `self` — `Some` values in `other` win.
    /// This is the CSS "override" model.
    pub fn merge(mut self, other: &Style) -> Self {
        macro_rules! override_field {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field.clone();
                }
            };
        }
        override_field!(background);
        override_field!(text_color);
        override_field!(font_size);
        override_field!(font_weight);
        override_field!(font_family);
        override_field!(text_align);
        override_field!(padding);
        override_field!(margin);
        override_field!(border);
        override_field!(width);
        override_field!(height);
        override_field!(opacity);
        override_field!(cursor);
        self
    }
}

// ── Theme ─────────────────────────────────────────────────────────────────────

/// Application-wide theme providing semantic color tokens.
///
/// Inspired by shadcn/ui and CSS custom properties.
#[derive(Debug, Clone)]
pub struct Theme {
    // Background layers
    pub bg: Color,
    pub bg_surface: Color,
    pub bg_elevated: Color,

    // Foreground / text
    pub fg: Color,
    pub fg_muted: Color,
    pub fg_subtle: Color,

    // Semantic colors
    pub accent: Color,
    pub accent_fg: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub danger_fg: Color,

    // Border
    pub border: Color,

    // Typography
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
    pub font_size_xl: f32,

    // Spacing scale
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
}

impl Theme {
    /// A dark theme inspired by VS Code / shadcn dark.
    pub fn dark() -> Self {
        Self {
            bg: Color::hex("#0d0f14"),
            bg_surface: Color::hex("#161820"),
            bg_elevated: Color::hex("#1e2130"),

            fg: Color::hex("#e2e4ec"),
            fg_muted: Color::hex("#8b8fa8"),
            fg_subtle: Color::hex("#555870"),

            accent: Color::hex("#5c7cfa"),
            accent_fg: Color::WHITE,
            success: Color::hex("#4fc08d"),
            warning: Color::hex("#fbbf24"),
            danger: Color::hex("#f87171"),
            danger_fg: Color::WHITE,

            border: Color::hex("#2a2d3e"),

            font_size_sm: 12.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 20.0,

            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 12.0,
        }
    }

    /// A light theme.
    pub fn light() -> Self {
        Self {
            bg: Color::hex("#ffffff"),
            bg_surface: Color::hex("#f5f5f5"),
            bg_elevated: Color::hex("#ebebeb"),

            fg: Color::hex("#111111"),
            fg_muted: Color::hex("#555555"),
            fg_subtle: Color::hex("#999999"),

            accent: Color::hex("#4263eb"),
            accent_fg: Color::WHITE,
            success: Color::hex("#2f9e44"),
            warning: Color::hex("#e67700"),
            danger: Color::hex("#e03131"),
            danger_fg: Color::WHITE,

            border: Color::hex("#dedede"),

            font_size_sm: 12.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
            font_size_xl: 20.0,

            radius_sm: 4.0,
            radius_md: 8.0,
            radius_lg: 12.0,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
