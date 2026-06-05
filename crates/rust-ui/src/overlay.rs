//! Overlay positioning — anchor-relative popup placement with flip/clamp.
//!
//! Used by Popover, Tooltip, Dropdown Menu, etc.

use crate::render::Rect;

/// Which edge of the anchor the popup attaches to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverSide {
    #[default]
    Bottom,
    Top,
    Left,
    Right,
}

/// Alignment along the anchor edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopoverAlign {
    #[default]
    Start,
    Center,
    End,
}

/// Place a popup relative to an anchor rect, flipping/clamping inside `viewport`.
pub fn place_popup(
    anchor: Rect,
    popup_w: f32,
    popup_h: f32,
    viewport: Rect,
    side: PopoverSide,
    align: PopoverAlign,
    gap: f32,
) -> Rect {
    let mut side = side;
    let mut rect = compute(anchor, popup_w, popup_h, side, align, gap);

    if matches!(side, PopoverSide::Bottom)
        && rect.y + rect.height > viewport.y + viewport.height
    {
        side = PopoverSide::Top;
        rect = compute(anchor, popup_w, popup_h, side, align, gap);
    } else if matches!(side, PopoverSide::Top) && rect.y < viewport.y {
        side = PopoverSide::Bottom;
        rect = compute(anchor, popup_w, popup_h, side, align, gap);
    }

    if matches!(side, PopoverSide::Right)
        && rect.x + rect.width > viewport.x + viewport.width
    {
        side = PopoverSide::Left;
        rect = compute(anchor, popup_w, popup_h, side, align, gap);
    } else if matches!(side, PopoverSide::Left) && rect.x < viewport.x {
        side = PopoverSide::Right;
        rect = compute(anchor, popup_w, popup_h, side, align, gap);
    }

    clamp_rect(rect, viewport, popup_w, popup_h)
}

fn compute(
    anchor: Rect,
    popup_w: f32,
    popup_h: f32,
    side: PopoverSide,
    align: PopoverAlign,
    gap: f32,
) -> Rect {
    let (x, y) = match side {
        PopoverSide::Bottom => (
            align_x(anchor, popup_w, align),
            anchor.y + anchor.height + gap,
        ),
        PopoverSide::Top => (
            align_x(anchor, popup_w, align),
            anchor.y - popup_h - gap,
        ),
        PopoverSide::Left => (
            anchor.x - popup_w - gap,
            align_y(anchor, popup_h, align),
        ),
        PopoverSide::Right => (
            anchor.x + anchor.width + gap,
            align_y(anchor, popup_h, align),
        ),
    };
    Rect::new(x, y, popup_w, popup_h)
}

fn align_x(anchor: Rect, popup_w: f32, align: PopoverAlign) -> f32 {
    match align {
        PopoverAlign::Start => anchor.x,
        PopoverAlign::Center => anchor.x + (anchor.width - popup_w) / 2.0,
        PopoverAlign::End => anchor.x + anchor.width - popup_w,
    }
}

fn align_y(anchor: Rect, popup_h: f32, align: PopoverAlign) -> f32 {
    match align {
        PopoverAlign::Start => anchor.y,
        PopoverAlign::Center => anchor.y + (anchor.height - popup_h) / 2.0,
        PopoverAlign::End => anchor.y + anchor.height - popup_h,
    }
}

fn clamp_rect(rect: Rect, viewport: Rect, popup_w: f32, popup_h: f32) -> Rect {
    let mut x = rect.x;
    let mut y = rect.y;
    if x + popup_w > viewport.x + viewport.width {
        x = viewport.x + viewport.width - popup_w;
    }
    if x < viewport.x {
        x = viewport.x;
    }
    if y + popup_h > viewport.y + viewport.height {
        y = viewport.y + viewport.height - popup_h;
    }
    if y < viewport.y {
        y = viewport.y;
    }
    Rect::new(x, y, popup_w, popup_h)
}

/// Build a generous viewport around an anchor for flip calculations.
pub fn viewport_around_anchor(anchor: Rect, padding: f32) -> Rect {
    Rect::new(
        anchor.x - padding,
        anchor.y - padding,
        anchor.width + padding * 2.0,
        anchor.height + padding * 2.0 + 480.0,
    )
}
