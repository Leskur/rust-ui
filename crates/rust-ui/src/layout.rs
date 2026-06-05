//! Layout engine — thin wrapper around `taffy` (CSS Flexbox / Grid).
//!
//! Widgets declare their layout constraints via `LayoutStyle`, the engine
//! computes positions and sizes, then passes them back to each widget's
//! `draw()` call via a `LayoutResult`.

pub use taffy::prelude::{
    AlignContent, AlignItems, FlexDirection, FlexWrap, JustifyContent, Style as TaffyStyle,
};

use crate::render::Rect;

/// Re-export taffy's dimension type under a friendlier name.
pub use taffy::prelude::Dimension;

/// Simplified layout style that maps to taffy.
#[derive(Debug, Clone, Default)]
pub struct LayoutStyle {
    pub display: DisplayMode,
    pub direction: FlexDirection,
    pub wrap: FlexWrap,
    pub justify_content: Option<JustifyContent>,
    pub align_items: Option<AlignItems>,
    pub gap: f32,
    pub padding: f32,
    pub width: SizeConstraint,
    pub height: SizeConstraint,
    pub flex_grow: f32,
    pub flex_shrink: f32,
}

/// Display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DisplayMode {
    #[default]
    Flex,
    Grid,
    Block,
    None,
}

/// Axis size constraint.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SizeConstraint {
    /// Size determined by content.
    #[default]
    Auto,
    /// Fixed pixel size.
    Px(f32),
    /// Percentage of parent.
    Percent(f32),
    /// Fill remaining space.
    Fill,
}

/// The computed position and size of a widget after layout.
#[derive(Debug, Clone, Copy)]
pub struct LayoutResult {
    pub rect: Rect,
}

impl LayoutStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn direction(mut self, d: FlexDirection) -> Self {
        self.direction = d;
        self
    }

    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn flex_grow(mut self, v: f32) -> Self {
        self.flex_grow = v;
        self
    }

    #[allow(dead_code)]
    fn to_taffy(&self) -> TaffyStyle {
        use taffy::prelude::*;

        let size_to_dim = |s: SizeConstraint| -> Dimension {
            match s {
                SizeConstraint::Auto => Dimension::Auto,
                SizeConstraint::Px(v) => Dimension::Length(v),
                SizeConstraint::Percent(p) => Dimension::Percent(p / 100.0),
                SizeConstraint::Fill => Dimension::Percent(1.0),
            }
        };

        TaffyStyle {
            display: match self.display {
                DisplayMode::Flex => taffy::prelude::Display::Flex,
                DisplayMode::Grid => taffy::prelude::Display::Grid,
                DisplayMode::Block => taffy::prelude::Display::Block,
                DisplayMode::None => taffy::prelude::Display::None,
            },
            flex_direction: self.direction,
            flex_wrap: self.wrap,
            gap: Size {
                width: length(self.gap),
                height: length(self.gap),
            },
            padding: Rect {
                left: length(self.padding),
                right: length(self.padding),
                top: length(self.padding),
                bottom: length(self.padding),
            },
            size: Size {
                width: size_to_dim(self.width),
                height: size_to_dim(self.height),
            },
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            justify_content: self.justify_content,
            align_items: self.align_items,
            ..Default::default()
        }
    }
}
