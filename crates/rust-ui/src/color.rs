//! Color type with CSS-compatible construction helpers.

/// An RGBA color with f32 components in [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const BLACK:       Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE:       Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    /// Construct from 8-bit RGBA components.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Construct from 8-bit RGB (fully opaque).
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    /// Construct from a CSS hex string: `"#5c7cfa"` or `"#5c7cfaff"`.
    ///
    /// Returns `Color::BLACK` if the string is invalid.
    pub fn hex(s: &str) -> Self {
        let s = s.trim_start_matches('#');
        let parse = |i: usize| -> u8 {
            u8::from_str_radix(&s[i..i + 2], 16).unwrap_or(0)
        };
        match s.len() {
            6 => Self::rgb(parse(0), parse(2), parse(4)),
            8 => Self::rgba(parse(0), parse(2), parse(4), parse(6)),
            _ => Self::BLACK,
        }
    }

    /// Linearly interpolate toward `other` by `t` (0.0 = self, 1.0 = other).
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }

    /// Return the same color with a different alpha.
    pub fn with_alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    /// Pack to `[r, g, b, a]` as `u8`.
    pub fn to_u8(self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }
}

impl Default for Color {
    fn default() -> Self { Self::TRANSPARENT }
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self { Self { r, g, b, a: 1.0 } }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self { Self { r, g, b, a } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_rgb() {
        let c = Color::hex("#ff8800");
        assert_eq!(c.to_u8(), [255, 136, 0, 255]);
    }

    #[test]
    fn lerp_halfway() {
        let a = Color::BLACK;
        let b = Color::WHITE;
        let mid = a.lerp(b, 0.5);
        assert!((mid.r - 0.5).abs() < 0.01);
    }
}
