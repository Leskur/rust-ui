//! Application window loop powered by winit + wgpu + vello.

use std::sync::Arc;

use pollster::FutureExt as _;
use vello::kurbo::Affine;
use vello::peniko;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{Key as WinitKey, NamedKey};
use winit::window::{Window, WindowId};

use rust_ui::animation::AnimationScheduler;
use rust_ui::event::Event as UiEvent;
use rust_ui::render::{Point, Rect};
use rust_ui::style::{CursorStyle, Theme};
use rust_ui::widget::Widget;

use crate::renderer::VelloRenderer;
use crate::text::TextEngine;

/// Run a rust-ui application. Blocks until the window is closed.
pub fn run(title: &str, width: u32, height: u32, root: impl Widget + 'static, theme: Theme) {
    let scheduler = std::sync::Arc::new(std::sync::Mutex::new(AnimationScheduler::new()));
    run_with_scheduler(title, width, height, root, theme, scheduler);
}

/// Run a rust-ui application with a custom AnimationScheduler.
/// This allows widgets to trigger animations that are rendered by the scheduler.
pub fn run_with_scheduler(
    title: &str,
    width: u32,
    height: u32,
    root: impl Widget + 'static,
    theme: Theme,
    scheduler: std::sync::Arc<std::sync::Mutex<AnimationScheduler>>,
) {
    // Register HTTP loader for Image widget (ureq, blocking, runs in background thread)
    rust_ui::widget::image::set_http_loader(|url| {
        use std::io::Read;
        let resp = ureq::get(url).call().map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        resp.into_reader()
            .read_to_end(&mut buf)
            .map_err(|e| e.to_string())?;
        Ok(buf)
    });

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App {
        title: title.to_string(),
        width,
        height,
        root: Box::new(root),
        theme,
        scheduler,
        state: None,
        cursor_pos: Point::new(0.0, 0.0),
        modifiers: winit::event::Modifiers::default(),
        focused_id: None,
    };
    event_loop.run_app(&mut app).expect("Event loop failed");
}

// ── GPU state (created after window is ready) ─────────────────────────────────

struct GpuState<'s> {
    window: Arc<Window>,
    surface: RenderSurface<'s>,
    renderer: Renderer,
    ctx: RenderContext,
    text: TextEngine,
    width: u32,
    height: u32,
}

// ── Application handler ───────────────────────────────────────────────────────

struct App {
    title: String,
    width: u32,
    height: u32,
    root: Box<dyn Widget>,
    theme: Theme,
    scheduler: std::sync::Arc<std::sync::Mutex<AnimationScheduler>>,
    state: Option<GpuState<'static>>,
    cursor_pos: Point,
    modifiers: winit::event::Modifiers,
    focused_id: Option<String>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(
                    winit::window::WindowAttributes::default()
                        .with_title(&self.title)
                        .with_inner_size(LogicalSize::new(self.width, self.height)),
                )
                .expect("Failed to create window"),
        );

        let mut ctx = RenderContext::new();
        let surface = ctx
            .create_surface(
                window.clone(),
                self.width,
                self.height,
                vello::wgpu::PresentMode::AutoVsync,
            )
            .block_on()
            .expect("Failed to create render surface");

        let device_handle = &ctx.devices[surface.dev_id];
        let renderer = Renderer::new(
            &device_handle.device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::area_only(),
                num_init_threads: std::num::NonZeroUsize::new(1),
            },
        )
        .expect("Failed to create vello renderer");

        let text = TextEngine::new();

        // Enable IME so the OS input method (e.g. Chinese/Japanese) can compose text.
        window.set_ime_allowed(true);

        // SAFETY: we keep window alive in GpuState for the lifetime of the app.
        let state: GpuState<'static> = unsafe {
            std::mem::transmute(GpuState {
                window,
                surface,
                renderer,
                ctx,
                text,
                width: self.width,
                height: self.height,
            })
        };
        self.state = Some(state);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                let Some(state) = self.state.as_mut() else { return };
                state.width = size.width.max(1);
                state.height = size.height.max(1);
                state
                    .ctx
                    .resize_surface(&mut state.surface, state.width, state.height);
                state.window.request_redraw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                let Some((w, h, window)) = self
                    .state
                    .as_ref()
                    .map(|s| (s.width as f32, s.height as f32, s.window.clone()))
                else {
                    return;
                };
                let pos = Point::new(position.x as f32, position.y as f32);
                self.cursor_pos = pos;
                let ui_event = UiEvent::MouseMove { pos };
                let bounds = Rect::new(0.0, 0.0, w, h);
                self.root.handle_event(&ui_event, bounds);

                // Update OS cursor based on widget under pointer
                let cursor = self.root.cursor_at((pos.x, pos.y), bounds);
                window.set_cursor(to_winit_cursor(cursor));
                window.request_redraw();
            }

            WindowEvent::ModifiersChanged(mods) => {
                self.modifiers = mods;
            }

            // IME composition: preedit text update and final commit
            WindowEvent::Ime(ime) => {
                use winit::event::Ime;
                let Some((w, h, window)) = self
                    .state
                    .as_ref()
                    .map(|s| (s.width as f32, s.height as f32, s.window.clone()))
                else {
                    return;
                };
                let bounds = Rect::new(0.0, 0.0, w, h);
                match ime {
                    Ime::Commit(text) => {
                        self.dispatch_key_event(&UiEvent::TextInput { text }, bounds);
                        window.request_redraw();
                    }
                    Ime::Preedit(text, cursor) => {
                        self.dispatch_key_event(&UiEvent::ImePreedit { text, cursor }, bounds);
                        window.request_redraw();
                    }
                    _ => {}
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                use rust_ui::event::{Key, Modifiers};
                if event.state != ElementState::Pressed {
                    return;
                }
                let Some((w, h, window)) = self
                    .state
                    .as_ref()
                    .map(|s| (s.width as f32, s.height as f32, s.window.clone()))
                else {
                    return;
                };
                let bounds = Rect::new(0.0, 0.0, w, h);
                let m = self.modifiers.state();
                let modifiers = Modifiers {
                    ctrl: m.control_key(),
                    shift: m.shift_key(),
                    alt: m.alt_key(),
                    meta: m.super_key(),
                };

                // Special named keys → KeyDown
                let ui_key = match &event.logical_key {
                    WinitKey::Named(NamedKey::Backspace) => Some(Key::Backspace),
                    WinitKey::Named(NamedKey::Delete) => Some(Key::Delete),
                    WinitKey::Named(NamedKey::Enter) => Some(Key::Enter),
                    WinitKey::Named(NamedKey::Escape) => Some(Key::Escape),
                    WinitKey::Named(NamedKey::Tab) => Some(Key::Tab),
                    WinitKey::Named(NamedKey::ArrowUp) => Some(Key::ArrowUp),
                    WinitKey::Named(NamedKey::ArrowDown) => Some(Key::ArrowDown),
                    WinitKey::Named(NamedKey::ArrowLeft) => Some(Key::ArrowLeft),
                    WinitKey::Named(NamedKey::ArrowRight) => Some(Key::ArrowRight),
                    WinitKey::Named(NamedKey::Home) => Some(Key::Home),
                    WinitKey::Named(NamedKey::End) => Some(Key::End),
                    _ => None,
                };
                if let Some(key) = ui_key {
                    self.dispatch_key_event(&UiEvent::KeyDown { key, modifiers }, bounds);
                    window.request_redraw();
                    return;
                }
                // Ctrl/Meta + char key → KeyDown (shortcuts like Ctrl+A)
                if modifiers.ctrl || modifiers.meta {
                    if let WinitKey::Character(s) = &event.logical_key {
                        if let Some(c) = s.chars().next() {
                            let key = Key::Char(c.to_ascii_lowercase());
                            self.dispatch_key_event(&UiEvent::KeyDown { key, modifiers }, bounds);
                            window.request_redraw();
                        }
                    }
                    return;
                }
                // Regular printable text — skip control characters
                if let Some(text) = event.text.as_ref() {
                    let s = text.to_string();
                    if s.chars().all(|c| !c.is_control()) && !s.is_empty() {
                        self.dispatch_key_event(&UiEvent::TextInput { text: s }, bounds);
                        window.request_redraw();
                    }
                }
            }

            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                let Some((w, h, window)) = self
                    .state
                    .as_ref()
                    .map(|s| (s.width as f32, s.height as f32, s.window.clone()))
                else {
                    return;
                };
                let (dx, dy) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };
                let pos = self.cursor_pos;
                let bounds = Rect::new(0.0, 0.0, w, h);
                self.root.handle_event(
                    &UiEvent::Scroll {
                        pos,
                        delta_x: dx,
                        delta_y: -dy,
                    },
                    bounds,
                );
                window.request_redraw();
            }

            WindowEvent::MouseInput {
                state: btn_state,
                button,
                ..
            } => {
                use winit::event::MouseButton;
                let Some((w, h, window)) = self
                    .state
                    .as_ref()
                    .map(|s| (s.width as f32, s.height as f32, s.window.clone()))
                else {
                    return;
                };
                let rb = match button {
                    MouseButton::Left => rust_ui::event::MouseButton::Left,
                    MouseButton::Right => rust_ui::event::MouseButton::Right,
                    MouseButton::Middle => rust_ui::event::MouseButton::Middle,
                    _ => return,
                };
                let pos = self.cursor_pos;
                let bounds = Rect::new(0.0, 0.0, w, h);

                // Focus follows primary-button press.
                if rb == rust_ui::event::MouseButton::Left && btn_state == ElementState::Pressed {
                    self.update_focus_from_pointer(pos, bounds);
                }
                let ui_event = match btn_state {
                    ElementState::Pressed => UiEvent::MouseDown { pos, button: rb },
                    ElementState::Released => UiEvent::MouseUp { pos, button: rb },
                };
                self.root.handle_event(&ui_event, bounds);
                self.ensure_focus_in_trap(bounds);
                window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                // Extract state to avoid double-borrow
                if let Some(st) = self.state.as_mut() {
                    Self::draw_frame(&mut self.root, &self.theme, &mut self.scheduler, st);
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            let mut needs_redraw = false;
            if let Ok(sched) = self.scheduler.lock() {
                if sched.has_active() {
                    needs_redraw = true;
                }
            }
            if !needs_redraw {
                let bounds = Rect::new(
                    0.0,
                    0.0,
                    state.width as f32,
                    state.height as f32,
                );
                needs_redraw = self.root.poll_redraw(bounds, &self.theme);
            }
            if needs_redraw {
                state.window.request_redraw();
            }
        }
    }
}

impl App {
    fn dispatch_key_event(&mut self, event: &UiEvent, bounds: Rect) {
        // Focus navigation: Tab / Shift+Tab cycles focus through focusable widgets.
        if let UiEvent::KeyDown { key, modifiers } = event {
            if *key == rust_ui::event::Key::Tab {
                let mut ids = Vec::new();
                collect_focusables(self.root.as_ref(), bounds, &self.theme, &mut ids);
                if ids.is_empty() {
                    return;
                }

                let cur_idx = self
                    .focused_id
                    .as_ref()
                    .and_then(|id| ids.iter().position(|x| x == id));

                let next_idx = if modifiers.shift {
                    match cur_idx {
                        Some(i) => (i + ids.len() - 1) % ids.len(),
                        None => ids.len() - 1,
                    }
                } else {
                    match cur_idx {
                        Some(i) => (i + 1) % ids.len(),
                        None => 0,
                    }
                };
                let next = ids[next_idx].clone();
                self.update_focus_to(Some(next), bounds);
                return;
            }
        }

        if let Some(id) = self.focused_id.clone() {
            if dispatch_to_id(self.root.as_mut(), &id, event, bounds, &self.theme) {
                self.ensure_focus_in_trap(bounds);
                return;
            }
        }
        self.root.handle_event(event, bounds);
        self.ensure_focus_in_trap(bounds);
    }

    fn update_focus_from_pointer(&mut self, pos: Point, bounds: Rect) {
        let next = hit_test_focusable(self.root.as_ref(), pos, bounds, &self.theme);
        self.update_focus_to(next, bounds);
    }

    fn update_focus_to(&mut self, next: Option<String>, bounds: Rect) {
        if next == self.focused_id {
            return;
        }

        if let Some(prev) = self.focused_id.take() {
            let _ = dispatch_to_id(
                self.root.as_mut(),
                &prev,
                &UiEvent::FocusLost,
                bounds,
                &self.theme,
            );
        }
        if let Some(next_id) = next.clone() {
            let _ = dispatch_to_id(
                self.root.as_mut(),
                &next_id,
                &UiEvent::FocusGained,
                bounds,
                &self.theme,
            );
        }
        self.focused_id = next;
    }

    /// When a focus trap (e.g. open Dialog) is active, keep focus inside it.
    fn ensure_focus_in_trap(&mut self, bounds: Rect) {
        let Some((trap, tb)) = find_focus_trap(self.root.as_ref(), bounds, &self.theme) else {
            return;
        };
        let mut ids = Vec::new();
        collect_focusables_in(trap, tb, &self.theme, &mut ids);
        if ids.is_empty() {
            return;
        }
        let in_trap = self
            .focused_id
            .as_ref()
            .map(|id| ids.iter().any(|x| x == id))
            .unwrap_or(false);
        if !in_trap {
            self.update_focus_to(Some(ids[0].clone()), bounds);
        }
    }

    fn draw_frame(
        root: &mut Box<dyn Widget>,
        theme: &Theme,
        scheduler: &std::sync::Arc<std::sync::Mutex<AnimationScheduler>>,
        state: &mut GpuState,
    ) {
        if let Ok(mut sched) = scheduler.lock() {
            sched.tick(1.0 / 60.0);
        }

        let mut scene = Scene::new();
        let bounds = Rect::new(0.0, 0.0, state.width as f32, state.height as f32);

        // Background
        let bg = theme.bg;
        scene.fill(
            peniko::Fill::NonZero,
            Affine::IDENTITY,
            peniko::Color::from_rgba8(
                (bg.r * 255.0) as u8,
                (bg.g * 255.0) as u8,
                (bg.b * 255.0) as u8,
                255,
            ),
            None,
            &vello::kurbo::Rect::new(0.0, 0.0, state.width as f64, state.height as f64),
        );

        // Widget tree
        let mut renderer = VelloRenderer::new(&mut scene, &mut state.text);
        root.draw(&mut renderer, bounds, theme);

        // Submit to GPU
        let device_handle = &state.ctx.devices[state.surface.dev_id];
        let surface_texture = state
            .surface
            .surface
            .get_current_texture()
            .expect("Failed to get surface texture");

        state
            .renderer
            .render_to_surface(
                &device_handle.device,
                &device_handle.queue,
                &scene,
                &surface_texture,
                &vello::RenderParams {
                    base_color: peniko::Color::BLACK,
                    width: state.width,
                    height: state.height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .expect("Failed to render");

        surface_texture.present();
        device_handle.device.poll(vello::wgpu::Maintain::Poll);
    }
}

/// Find the top-most widget that traps focus (e.g. an open Dialog).
fn find_focus_trap<'a>(
    widget: &'a dyn Widget,
    bounds: Rect,
    theme: &Theme,
) -> Option<(&'a dyn Widget, Rect)> {
    if widget.blocks_focus_outside() {
        return Some((widget, bounds));
    }
    for (child, cb) in widget.layout_children(bounds, theme).into_iter().rev() {
        if let Some(found) = find_focus_trap(child, cb, theme) {
            return Some(found);
        }
    }
    None
}

fn hit_test_focusable(
    widget: &dyn Widget,
    pos: Point,
    bounds: Rect,
    theme: &Theme,
) -> Option<String> {
    if let Some((trap, tb)) = find_focus_trap(widget, bounds, theme) {
        return hit_test_focusable_in(trap, pos, tb, theme);
    }
    hit_test_focusable_in(widget, pos, bounds, theme)
}

fn hit_test_focusable_in(
    widget: &dyn Widget,
    pos: Point,
    bounds: Rect,
    theme: &Theme,
) -> Option<String> {
    if !bounds.contains(pos.x, pos.y) {
        return None;
    }
    for (child, cb) in widget.layout_children(bounds, theme) {
        if cb.contains(pos.x, pos.y) {
            if let Some(id) = hit_test_focusable_in(child, pos, cb, theme) {
                return Some(id);
            }
        }
    }
    if widget.focusable() {
        Some(widget.id().to_string())
    } else {
        None
    }
}

fn collect_focusables(widget: &dyn Widget, bounds: Rect, theme: &Theme, out: &mut Vec<String>) {
    if let Some((trap, tb)) = find_focus_trap(widget, bounds, theme) {
        collect_focusables_in(trap, tb, theme, out);
        return;
    }
    collect_focusables_in(widget, bounds, theme, out);
}

fn collect_focusables_in(widget: &dyn Widget, bounds: Rect, theme: &Theme, out: &mut Vec<String>) {
    for (child, cb) in widget.layout_children(bounds, theme) {
        collect_focusables_in(child, cb, theme, out);
    }
    if widget.focusable() {
        out.push(widget.id().to_string());
    }
}

fn dispatch_to_id(
    widget: &mut dyn Widget,
    target_id: &str,
    event: &UiEvent,
    bounds: Rect,
    theme: &Theme,
) -> bool {
    if widget.id() == target_id {
        widget.handle_event(event, bounds);
        return true;
    }
    for (child, cb) in widget.layout_children_mut(bounds, theme) {
        if dispatch_to_id(child, target_id, event, cb, theme) {
            return true;
        }
    }
    false
}

fn to_winit_cursor(c: CursorStyle) -> winit::window::CursorIcon {
    use winit::window::CursorIcon;
    match c {
        CursorStyle::Default => CursorIcon::Default,
        CursorStyle::Pointer => CursorIcon::Pointer,
        CursorStyle::Text => CursorIcon::Text,
        CursorStyle::NotAllowed => CursorIcon::NotAllowed,
        CursorStyle::Grab => CursorIcon::Grab,
        CursorStyle::Crosshair => CursorIcon::Crosshair,
    }
}
