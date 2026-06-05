//! Svg widget — renders SVG files with custom size and color.
//!
//! SVG files are rasterized using resvg and rendered as images.
//!
//! ```rust,ignore
//! svg("assets/icon.svg").width(32.0).height(32.0).color(Color::hex("#6366f1"))
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::color::Color;
use crate::render::{Rect, Renderer};
use crate::style::Theme;
use crate::widget::Widget;

// ── Global SVG cache ───────────────────────────────────────────────────────────

#[derive(Clone)]
struct RasterizedSvg {
    data:   Arc<Vec<u8>>, // RGBA8 bytes
    width:  u32,
    height: u32,
}

#[derive(Clone)]
enum CacheEntry {
    Loading,
    Loaded(RasterizedSvg),
    Error(String),
}

fn svg_cache() -> &'static Mutex<HashMap<String, CacheEntry>> {
    static CACHE: OnceLock<Mutex<HashMap<String, CacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

// ── Svg widget ────────────────────────────────────────────────────────────────

pub struct Svg {
    id:     String,
    src:    String,
    width:  Option<f32>,
    height: Option<f32>,
    color:  Option<Color>,
}

impl Svg {
    pub fn new(src: impl Into<String>) -> Self {
        let src = src.into();
        Self::spawn_load(src.clone());
        Self {
            id:     uuid(),
            src,
            width:  None,
            height: None,
            color:  None,
        }
    }

    pub fn width(mut self, w: f32) -> Self { self.width = Some(w); self }
    pub fn height(mut self, h: f32) -> Self { self.height = Some(h); self }
    pub fn color(mut self, c: Color) -> Self { self.color = Some(c); self }

    /// Spawn a background thread to load + rasterize the SVG (if not already cached).
    fn spawn_load(src: String) {
        let mut cache = svg_cache().lock().unwrap();
        if cache.contains_key(&src) {
            return;
        }
        cache.insert(src.clone(), CacheEntry::Loading);
        drop(cache);

        std::thread::spawn(move || {
            let result = Self::load_and_rasterize(&src);
            let entry = match result {
                Ok(svg) => CacheEntry::Loaded(svg),
                Err(e)  => CacheEntry::Error(e),
            };
            svg_cache().lock().unwrap().insert(src, entry);
        });
    }

    fn load_and_rasterize(src: &str) -> Result<RasterizedSvg, String> {
        let svg_data = if src.starts_with("http://") || src.starts_with("https://") {
            crate::widget::image::fetch_bytes(src)?
        } else {
            std::fs::read(src).map_err(|e| format!("read file: {e}"))?
        };

        // Parse SVG with resvg
        let tree = resvg::usvg::Tree::from_data(&svg_data, &Default::default())
            .map_err(|e| format!("parse SVG: {e}"))?;

        // Get original size
        let size = tree.size();
        let width = size.width() as u32;
        let height = size.height() as u32;

        // Rasterize to RGBA
        let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)
            .ok_or("create pixmap")?;
        let transform = resvg::tiny_skia::Transform::identity();
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        Ok(RasterizedSvg {
            data:   Arc::new(pixmap.data().to_vec()),
            width,
            height,
        })
    }

    fn cache_entry(&self) -> Option<CacheEntry> {
        svg_cache().lock().ok()?.get(&self.src).cloned()
    }
}

impl Widget for Svg {
    fn id(&self) -> &str { &self.id }

    fn intrinsic_size(&self, _theme: &Theme) -> (f32, f32) {
        let w = self.width.unwrap_or(120.0);
        let h = self.height.unwrap_or(120.0);
        (w, h)
    }

    fn draw(&self, renderer: &mut dyn Renderer, bounds: Rect, theme: &Theme) {
        let w = self.width.unwrap_or(bounds.width);
        let h = self.height.unwrap_or(bounds.height);
        let bounds = Rect::new(bounds.x, bounds.y, w, h);

        match self.cache_entry() {
            Some(CacheEntry::Loaded(svg)) => {
                renderer.draw_image(&svg.data, svg.width, svg.height, bounds);
            }
            Some(CacheEntry::Error(_)) => {
                // Error placeholder: grey box with an ✕
                renderer.fill_rect(bounds, theme.bg_elevated, crate::style::Corners::all(4.0));
                renderer.stroke_rect(bounds, theme.border, 1.0, crate::style::Corners::all(4.0));
                let cx = bounds.x + bounds.width  / 2.0 - 8.0;
                let cy = bounds.y + bounds.height / 2.0 - 7.0;
                let opts = crate::render::TextOptions {
                    font_size: 14.0,
                    color: theme.fg_muted,
                    ..Default::default()
                };
                renderer.draw_text("✕", crate::render::Point::new(cx, cy), &opts);
            }
            _ => {
                // Loading placeholder: pulsing grey box
                renderer.fill_rect(bounds, theme.bg_elevated, crate::style::Corners::all(4.0));
            }
        }
    }
}

/// Shorthand constructor.
pub fn svg(src: impl Into<String>) -> Svg {
    Svg::new(src)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("svg-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
