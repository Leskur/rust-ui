//! Animation scheduler.
//!
//! # Design
//! The scheduler owns all active animations. Each frame the runtime calls
//! `scheduler.tick(dt)`, which advances every active animation and marks
//! dirty widgets. Widgets never manage time themselves.
//!
//! ```rust,ignore
//! // In your widget's event handler:
//! scheduler.animate_to(&self.id, "opacity", 0.0, Easing::EaseOut, 0.3);
//!
//! // In your widget's draw method:
//! let opacity = scheduler.get(&self.id, "opacity").unwrap_or(1.0);
//! ```

use std::collections::HashMap;

/// Easing functions — same names as CSS.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Spring-style overshoot.
    Spring {
        stiffness: f32,
        damping: f32,
    },
}

impl Easing {
    /// Map a linear `t` in [0, 1] to an eased value.
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
            Easing::Spring { .. } => {
                // Simplified exponential approximation
                1.0 - (-t * 5.0).exp() * (1.0 - t)
            }
        }
    }
}

/// A single animating value.
#[derive(Debug, Clone)]
pub struct Animation {
    pub from: f32,
    pub to: f32,
    pub elapsed: f32,
    pub duration: f32,
    pub easing: Easing,
    pub is_complete: bool,
}

impl Animation {
    pub fn new(from: f32, to: f32, duration: f32, easing: Easing) -> Self {
        Self {
            from,
            to,
            elapsed: 0.0,
            duration,
            easing,
            is_complete: false,
        }
    }

    /// Advance by `dt` seconds. Returns current interpolated value.
    pub fn tick(&mut self, dt: f32) -> f32 {
        if self.is_complete {
            return self.to;
        }
        self.elapsed += dt;
        if self.elapsed >= self.duration {
            self.elapsed = self.duration;
            self.is_complete = true;
        }
        let t = self.elapsed / self.duration;
        let t_eased = self.easing.apply(t);
        self.from + (self.to - self.from) * t_eased
    }

    pub fn current(&self) -> f32 {
        if self.is_complete {
            return self.to;
        }
        let t = (self.elapsed / self.duration).clamp(0.0, 1.0);
        let t_eased = self.easing.apply(t);
        self.from + (self.to - self.from) * t_eased
    }
}

/// Key identifying an animated property on a widget.
/// e.g. `("btn-1", "opacity")` or `("switch-proxy", "thumb_x")`
type AnimKey = (String, &'static str);

/// The global animation scheduler.
///
/// Owned by the application runtime, passed by mutable reference each frame.
#[derive(Default)]
pub struct AnimationScheduler {
    animations: HashMap<AnimKey, Animation>,
    /// Values snapshot after the last tick — widgets read from here.
    values: HashMap<AnimKey, f32>,
}

impl AnimationScheduler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start or restart an animation for a widget property.
    pub fn animate_to(
        &mut self,
        widget_id: impl Into<String>,
        property: &'static str,
        to: f32,
        easing: Easing,
        duration_secs: f32,
    ) {
        let key = (widget_id.into(), property);
        let from = self.values.get(&key).copied().unwrap_or(to);
        if (from - to).abs() < 0.001 {
            return;
        } // already there
        self.animations
            .insert(key, Animation::new(from, to, duration_secs, easing));
    }

    /// Set a value instantly (no animation).
    pub fn set(&mut self, widget_id: impl Into<String>, property: &'static str, value: f32) {
        let key = (widget_id.into(), property);
        self.values.insert(key.clone(), value);
        self.animations.remove(&key);
    }

    /// Read the current value of a property (interpolated).
    pub fn get(&self, widget_id: &str, property: &'static str) -> Option<f32> {
        self.values.get(&(widget_id.to_string(), property)).copied()
    }

    /// Returns `true` if any animation is still running.
    pub fn has_active(&self) -> bool {
        self.animations.values().any(|a| !a.is_complete)
    }

    /// Advance all animations by `dt` seconds.
    /// Call this once per frame from the runtime.
    pub fn tick(&mut self, dt: f32) {
        let mut finished = Vec::new();
        for (key, anim) in &mut self.animations {
            let v = anim.tick(dt);
            self.values.insert(key.clone(), v);
            if anim.is_complete {
                finished.push(key.clone());
            }
        }
        for key in finished {
            self.animations.remove(&key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_completes() {
        let mut a = Animation::new(0.0, 1.0, 0.3, Easing::Linear);
        a.tick(0.15);
        assert!((a.current() - 0.5).abs() < 0.01);
        a.tick(0.15);
        assert!(a.is_complete);
        assert_eq!(a.current(), 1.0);
    }

    #[test]
    fn scheduler_animate_to() {
        let mut s = AnimationScheduler::new();
        s.set("w1", "opacity", 1.0);
        s.animate_to("w1", "opacity", 0.0, Easing::Linear, 1.0);
        s.tick(0.5);
        let v = s.get("w1", "opacity").unwrap();
        assert!((v - 0.5).abs() < 0.01);
    }
}
