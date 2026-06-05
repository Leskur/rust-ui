//! Badge — small status label (shadcn/ui inspired).
//!
//! ```rust,ignore
//! badge("New");
//! badge("Beta").variant(BadgeVariant::Secondary);
//! badge("Error").variant(BadgeVariant::Destructive);
//! ```

use crate::render::{Point, Rect, Renderer, TextOptions};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

const PAD_X: f32 = 10.0;
const PAD_Y: f32 = 4.0;

/// Visual style for a badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

pub struct Badge {
    id: String,
    label: String,
    variant: BadgeVariant,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: uuid(),
            label: label.into(),
            variant: BadgeVariant::Default,
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    fn colors(&self, theme: &Theme) -> (crate::color::Color, crate::color::Color) {
        use crate::color::Color;
        match self.variant {
            BadgeVariant::Default => (theme.accent, theme.accent_fg),
            BadgeVariant::Secondary => (theme.bg_elevated, theme.fg),
            BadgeVariant::Destructive => (theme.danger, theme.danger_fg),
            BadgeVariant::Outline => (Color::TRANSPARENT, theme.fg),
        }
    }
}

impl Widget for Badge {
    fn id(&self) -> &str {
        &self.id
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let opts = TextOptions {
            font_size: theme.font_size_sm,
            color: theme.fg,
            bold: true,
            ..Default::default()
        };
        let (tw, th, _) = renderer.measure_text(&self.label, &opts);
        let w = tw + PAD_X * 2.0;
        let h = th + PAD_Y * 2.0;
        let pill = Rect::new(bounds.x, bounds.y + (bounds.height - h) / 2.0, w, h);

        let (bg, fg) = self.colors(theme);
        renderer.fill_rect(pill, bg, Corners::all(h / 2.0));
        if self.variant == BadgeVariant::Outline {
            renderer.stroke_rect(pill, theme.border, 1.0, Corners::all(h / 2.0));
        }

        let text_opts = TextOptions {
            font_size: theme.font_size_sm,
            color: fg,
            bold: true,
            ..Default::default()
        };
        renderer.draw_text(
            &self.label,
            Point::new(pill.x + PAD_X, pill.y + PAD_Y),
            &text_opts,
        );
    }

    fn intrinsic_size(&self, theme: &Theme) -> (f32, f32) {
        let char_w = theme.font_size_sm * 0.6;
        let tw = self.label.chars().count() as f32 * char_w;
        let h = theme.font_size_sm + PAD_Y * 2.0;
        (tw + PAD_X * 2.0, h.max(22.0))
    }
}

pub fn badge(label: impl Into<String>) -> Badge {
    Badge::new(label)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("badge-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
