//! Progress — horizontal progress indicator (shadcn/ui inspired).
//!
//! ```rust,ignore
//! progress().value(33.0).width(320.0);
//! progress().indeterminate().width(320.0);
//! ```

use std::time::Instant;

use crate::render::{Rect, Renderer};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

const DEFAULT_WIDTH: f32 = 240.0;
const DEFAULT_HEIGHT: f32 = 8.0;
const INDETERMINATE_CYCLE_SECS: f32 = 1.4;

/// Horizontal progress bar with determinate or indeterminate modes.
pub struct Progress {
    id: String,
    /// `None` = indeterminate; `Some(0.0..=1.0)` = fill fraction.
    value: Option<f32>,
    width: Option<f32>,
    height: f32,
    anim_start: Instant,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            id: uuid(),
            value: None,
            width: None,
            height: DEFAULT_HEIGHT,
            anim_start: Instant::now(),
        }
    }

    /// Set fill amount as a percentage (`0`–`100`).
    pub fn value(mut self, percent: f32) -> Self {
        self.value = Some((percent / 100.0).clamp(0.0, 1.0));
        self
    }

    /// Indeterminate loading state (animated).
    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self.anim_start = Instant::now();
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h.max(2.0);
        self
    }

    fn track_width(&self, bounds: Rect) -> f32 {
        self.width.unwrap_or(DEFAULT_WIDTH).min(bounds.width.max(DEFAULT_WIDTH))
    }

    fn track_rect(&self, bounds: Rect) -> Rect {
        let w = self.track_width(bounds);
        let h = self.height;
        Rect::new(
            bounds.x,
            bounds.y + (bounds.height - h) / 2.0,
            w,
            h,
        )
    }

    fn indeterminate_phase(&self) -> f32 {
        let t = self.anim_start.elapsed().as_secs_f32() % INDETERMINATE_CYCLE_SECS;
        t / INDETERMINATE_CYCLE_SECS
    }
}

impl Widget for Progress {
    fn id(&self) -> &str {
        &self.id
    }

    fn wants_redraw(&self) -> bool {
        self.value.is_none()
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let track = self.track_rect(bounds);
        let radius = Corners::all(track.height / 2.0);

        renderer.fill_rect(track, theme.bg_elevated, radius);
        renderer.stroke_rect(track, theme.border.with_alpha(0.5), 1.0, radius);

        match self.value {
            Some(fraction) => {
                let fill_w = (track.width * fraction).max(if fraction > 0.0 { track.height } else { 0.0 });
                if fill_w > 0.0 {
                    renderer.fill_rect(
                        Rect::new(track.x, track.y, fill_w.min(track.width), track.height),
                        theme.accent,
                        radius,
                    );
                }
            }
            None => {
                let seg_w = track.width * 0.35;
                let phase = self.indeterminate_phase();
                // Slide segment across the track (enter from left, exit right).
                let travel = track.width + seg_w;
                let x = track.x - seg_w + travel * phase;
                let seg = Rect::new(x, track.y, seg_w, track.height);
                let clip = track;
                renderer.push_clip(clip);
                renderer.fill_rect(seg, theme.accent, radius);
                renderer.pop_clip();
            }
        }
    }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        (self.width.unwrap_or(DEFAULT_WIDTH), self.height.max(16.0))
    }
}

/// Shorthand — indeterminate progress bar.
pub fn progress() -> Progress {
    Progress::new()
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("progress-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
