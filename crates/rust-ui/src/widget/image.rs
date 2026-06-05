//! Image widget — renders local files or remote URLs with async loading.
//!
//! Images are loaded in a background thread. During loading a placeholder is shown.
//! Decoded RGBA bytes are cached globally by URL/path to avoid redundant downloads.
//!
//! ```rust,ignore
//! image("https://example.com/photo.png")
//!     .width(320.0)
//!     .height(240.0)
//!     .fit(ObjectFit::Cover)
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::render::{Rect, Renderer};
use crate::style::{Corners, Theme};
use crate::widget::Widget;

// ── Global image cache ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct DecodedImage {
    pub data: Arc<Vec<u8>>, // RGBA8 bytes
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
enum CacheEntry {
    Loading,
    Loaded(DecodedImage),
    Error(String),
}

fn image_cache() -> &'static Mutex<HashMap<String, CacheEntry>> {
    static CACHE: OnceLock<Mutex<HashMap<String, CacheEntry>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

// ── HTTP loader (pluggable by backend) ────────────────────────────────────────

type HttpLoaderFn = Arc<dyn Fn(&str) -> Result<Vec<u8>, String> + Send + Sync>;

fn http_loader() -> &'static Mutex<Option<HttpLoaderFn>> {
    static LOADER: OnceLock<Mutex<Option<HttpLoaderFn>>> = OnceLock::new();
    LOADER.get_or_init(|| Mutex::new(None))
}

/// Register an HTTP loader function.
///
/// Call this once at startup (e.g. from `rust-ui-wgpu`'s `run`) before any
/// `image("https://...")` widget is constructed.
///
/// ```rust,ignore
/// rust_ui::widget::image::set_http_loader(|url| {
///     let resp = ureq::get(url).call().map_err(|e| e.to_string())?;
///     let mut buf = Vec::new();
///     resp.into_reader().read_to_end(&mut buf).map_err(|e| e.to_string())?;
///     Ok(buf)
/// });
/// ```
pub fn set_http_loader(f: impl Fn(&str) -> Result<Vec<u8>, String> + Send + Sync + 'static) {
    *http_loader().lock().unwrap() = Some(Arc::new(f));
}

/// Fetch raw bytes from an HTTP/HTTPS URL using the registered loader.
pub fn fetch_bytes(url: &str) -> Result<Vec<u8>, String> {
    // Clone the Arc so we can call without holding the lock
    let f = http_loader().lock().unwrap().clone();
    match f {
        Some(f) => f(url),
        None => Err("No HTTP loader registered. Call rust_ui::widget::image::set_http_loader() from your backend.".into()),
    }
}

// ── ObjectFit ─────────────────────────────────────────────────────────────────

/// How the image fills its bounds (mirrors CSS object-fit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectFit {
    /// Stretch to fill (may distort).
    Fill,
    /// Scale uniformly to fit inside bounds (letterbox).
    #[default]
    Contain,
    /// Scale uniformly to cover bounds (may crop).
    Cover,
    /// Do not scale; display at natural size.
    None,
}

// ── Image widget ──────────────────────────────────────────────────────────────

pub struct Image {
    id: String,
    src: String,
    width: Option<f32>,
    height: Option<f32>,
    fit: ObjectFit,
    radius: f32,
}

impl Image {
    pub fn new(src: impl Into<String>) -> Self {
        let src = src.into();
        Self::spawn_load(src.clone());
        Self {
            id: uuid(),
            src,
            width: None,
            height: None,
            fit: ObjectFit::default(),
            radius: 0.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }
    pub fn fit(mut self, f: ObjectFit) -> Self {
        self.fit = f;
        self
    }
    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
        self
    }

    /// Spawn a background thread to load + decode the image (if not already cached).
    fn spawn_load(src: String) {
        let mut cache = image_cache().lock().unwrap();
        if cache.contains_key(&src) {
            return;
        }
        cache.insert(src.clone(), CacheEntry::Loading);
        drop(cache);

        std::thread::spawn(move || {
            let result = Self::load_and_decode(&src);
            let entry = match result {
                Ok(img) => CacheEntry::Loaded(img),
                Err(e) => CacheEntry::Error(e),
            };
            image_cache().lock().unwrap().insert(src, entry);
        });
    }

    fn load_and_decode(src: &str) -> Result<DecodedImage, String> {
        let bytes: Vec<u8> = if src.starts_with("http://") || src.starts_with("https://") {
            fetch_bytes(src)?
        } else {
            std::fs::read(src).map_err(|e| format!("read file: {e}"))?
        };

        let img = image::load_from_memory(&bytes)
            .map_err(|e| format!("decode: {e}"))?
            .into_rgba8();
        let (w, h) = img.dimensions();
        Ok(DecodedImage {
            data: Arc::new(img.into_raw()),
            width: w,
            height: h,
        })
    }

    /// Compute destination rect inside `bounds` based on ObjectFit + natural size.
    fn fit_rect(&self, bounds: Rect, img_w: u32, img_h: u32) -> Rect {
        let (iw, ih) = (img_w as f32, img_h as f32);
        let (bw, bh) = (bounds.width, bounds.height);
        match self.fit {
            ObjectFit::Fill => bounds,
            ObjectFit::None => {
                let w = iw.min(bw);
                let h = ih.min(bh);
                Rect::new(bounds.x + (bw - w) / 2.0, bounds.y + (bh - h) / 2.0, w, h)
            }
            ObjectFit::Contain => {
                let scale = (bw / iw).min(bh / ih);
                let w = iw * scale;
                let h = ih * scale;
                Rect::new(bounds.x + (bw - w) / 2.0, bounds.y + (bh - h) / 2.0, w, h)
            }
            ObjectFit::Cover => {
                let scale = (bw / iw).max(bh / ih);
                let w = iw * scale;
                let h = ih * scale;
                Rect::new(bounds.x + (bw - w) / 2.0, bounds.y + (bh - h) / 2.0, w, h)
            }
        }
    }

    fn cache_entry(&self) -> Option<CacheEntry> {
        image_cache().lock().ok()?.get(&self.src).cloned()
    }
}

impl Widget for Image {
    fn id(&self) -> &str {
        &self.id
    }

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
            Some(CacheEntry::Loaded(img)) => {
                let dest = self.fit_rect(bounds, img.width, img.height);
                if self.radius > 0.0 {
                    renderer.push_clip(bounds);
                }
                renderer.draw_image(&img.data, img.width, img.height, dest);
                if self.radius > 0.0 {
                    renderer.pop_clip();
                }
            }
            Some(CacheEntry::Error(err)) => {
                // Error placeholder: grey box with an ✕
                renderer.fill_rect(bounds, theme.bg_elevated, Corners::all(self.radius));
                renderer.stroke_rect(bounds, theme.border, 1.0, Corners::all(self.radius));
                let cx = bounds.x + bounds.width / 2.0 - 8.0;
                let cy = bounds.y + bounds.height / 2.0 - 7.0;
                let opts = crate::render::TextOptions {
                    font_size: 14.0,
                    color: theme.fg_muted,
                    ..Default::default()
                };
                renderer.draw_text("✕", crate::render::Point::new(cx, cy), &opts);
                // Read `err` so it's not optimized away; backends can surface it later.
                let _ = err;
            }
            _ => {
                // Loading placeholder: pulsing grey box
                renderer.fill_rect(bounds, theme.bg_elevated, Corners::all(self.radius));
            }
        }
    }
}

/// Shorthand constructor.
pub fn image(src: impl Into<String>) -> Image {
    Image::new(src)
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CTR: AtomicU64 = AtomicU64::new(0);
    format!("image-{}", CTR.fetch_add(1, Ordering::Relaxed))
}
