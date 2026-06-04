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
pub fn run(
    title:  &str,
    width:  u32,
    height: u32,
    root:   impl Widget + 'static,
    theme:  Theme,
) {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App {
        title:      title.to_string(),
        width,
        height,
        root:       Box::new(root),
        theme,
        scheduler:  AnimationScheduler::new(),
        state:      None,
        cursor_pos: Point::new(0.0, 0.0),
        modifiers:  winit::event::Modifiers::default(),
    };
    event_loop.run_app(&mut app).expect("Event loop failed");
}

// ── GPU state (created after window is ready) ─────────────────────────────────

struct GpuState<'s> {
    window:   Arc<Window>,
    surface:  RenderSurface<'s>,
    renderer: Renderer,
    ctx:      RenderContext,
    text:     TextEngine,
    width:    u32,
    height:   u32,
}

// ── Application handler ───────────────────────────────────────────────────────

struct App {
    title:      String,
    width:      u32,
    height:     u32,
    root:       Box<dyn Widget>,
    theme:      Theme,
    scheduler:  AnimationScheduler,
    state:      Option<GpuState<'static>>,
    cursor_pos: Point,
    modifiers:  winit::event::Modifiers,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.state.is_some() { return; }

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
                use_cpu:        false,
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
                width:  self.width,
                height: self.height,
            })
        };
        self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let Some(state) = &mut self.state else { return };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::Resized(size) => {
                state.width  = size.width.max(1);
                state.height = size.height.max(1);
                state.ctx.resize_surface(&mut state.surface, state.width, state.height);
                state.window.request_redraw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                let pos = Point::new(position.x as f32, position.y as f32);
                self.cursor_pos = pos;
                let ui_event = UiEvent::MouseMove { pos };
                let bounds = Rect::new(0.0, 0.0, state.width as f32, state.height as f32);
                self.root.handle_event(&ui_event, bounds);

                // Update OS cursor based on widget under pointer
                let cursor = self.root.cursor_at((pos.x, pos.y), bounds);
                state.window.set_cursor(to_winit_cursor(cursor));
                state.window.request_redraw();
            }

            WindowEvent::ModifiersChanged(mods) => {
                self.modifiers = mods;
            }

            // IME composition: preedit text update and final commit
            WindowEvent::Ime(ime) => {
                use winit::event::Ime;
                let bounds = Rect::new(0.0, 0.0, state.width as f32, state.height as f32);
                match ime {
                    Ime::Commit(text) => {
                        self.root.handle_event(&UiEvent::TextInput { text }, bounds);
                        state.window.request_redraw();
                    }
                    Ime::Preedit(text, cursor) => {
                        self.root.handle_event(&UiEvent::ImePreedit { text, cursor }, bounds);
                        state.window.request_redraw();
                    }
                    _ => {}
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                use rust_ui::event::{Key, Modifiers};
                if event.state != ElementState::Pressed { return; }
                let bounds = Rect::new(0.0, 0.0, state.width as f32, state.height as f32);
                let m = self.modifiers.state();
                let modifiers = Modifiers {
                    ctrl:  m.control_key(),
                    shift: m.shift_key(),
                    alt:   m.alt_key(),
                    meta:  m.super_key(),
                };

                // Special named keys → KeyDown
                let ui_key = match &event.logical_key {
                    WinitKey::Named(NamedKey::Backspace)  => Some(Key::Backspace),
                    WinitKey::Named(NamedKey::Delete)     => Some(Key::Delete),
                    WinitKey::Named(NamedKey::Enter)      => Some(Key::Enter),
                    WinitKey::Named(NamedKey::Escape)     => Some(Key::Escape),
                    WinitKey::Named(NamedKey::Tab)        => Some(Key::Tab),
                    WinitKey::Named(NamedKey::ArrowUp)    => Some(Key::ArrowUp),
                    WinitKey::Named(NamedKey::ArrowDown)  => Some(Key::ArrowDown),
                    WinitKey::Named(NamedKey::ArrowLeft)  => Some(Key::ArrowLeft),
                    WinitKey::Named(NamedKey::ArrowRight) => Some(Key::ArrowRight),
                    WinitKey::Named(NamedKey::Home)       => Some(Key::Home),
                    WinitKey::Named(NamedKey::End)        => Some(Key::End),
                    _ => None,
                };
                if let Some(key) = ui_key {
                    self.root.handle_event(&UiEvent::KeyDown { key, modifiers }, bounds);
                    state.window.request_redraw();
                    return;
                }
                // Ctrl/Meta + char key → KeyDown (shortcuts like Ctrl+A)
                if modifiers.ctrl || modifiers.meta {
                    if let WinitKey::Character(s) = &event.logical_key {
                        if let Some(c) = s.chars().next() {
                            let key = Key::Char(c.to_ascii_lowercase());
                            self.root.handle_event(&UiEvent::KeyDown { key, modifiers }, bounds);
                            state.window.request_redraw();
                        }
                    }
                    return;
                }
                // Regular printable text — skip control characters
                if let Some(text) = event.text.as_ref() {
                    let s = text.to_string();
                    if s.chars().all(|c| !c.is_control()) && !s.is_empty() {
                        self.root.handle_event(&UiEvent::TextInput { text: s }, bounds);
                        state.window.request_redraw();
                    }
                }
            }

            WindowEvent::MouseInput { state: btn_state, button, .. } => {
                use winit::event::MouseButton;
                let rb = match button {
                    MouseButton::Left   => rust_ui::event::MouseButton::Left,
                    MouseButton::Right  => rust_ui::event::MouseButton::Right,
                    MouseButton::Middle => rust_ui::event::MouseButton::Middle,
                    _                   => return,
                };
                let pos = self.cursor_pos;
                let ui_event = match btn_state {
                    ElementState::Pressed  => UiEvent::MouseDown { pos, button: rb },
                    ElementState::Released => UiEvent::MouseUp   { pos, button: rb },
                };
                let bounds = Rect::new(0.0, 0.0, state.width as f32, state.height as f32);
                self.root.handle_event(&ui_event, bounds);
                state.window.request_redraw();
            }

            WindowEvent::RedrawRequested => {
                // Extract state to avoid double-borrow
                if let Some(st) = self.state.as_mut() {
                    Self::draw_frame(
                        &mut self.root,
                        &self.theme,
                        &mut self.scheduler,
                        st,
                    );
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(state) = &self.state {
            if self.scheduler.has_active() {
                state.window.request_redraw();
            }
        }
    }
}

impl App {
    fn draw_frame(
        root:      &mut Box<dyn Widget>,
        theme:     &Theme,
        scheduler: &mut AnimationScheduler,
        state:     &mut GpuState,
    ) {
        scheduler.tick(1.0 / 60.0);

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
        let surface_texture = state.surface.surface
            .get_current_texture()
            .expect("Failed to get surface texture");

        state.renderer
            .render_to_surface(
                &device_handle.device,
                &device_handle.queue,
                &scene,
                &surface_texture,
                &vello::RenderParams {
                    base_color: peniko::Color::BLACK,
                    width:      state.width,
                    height:     state.height,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .expect("Failed to render");

        surface_texture.present();
        device_handle.device.poll(vello::wgpu::Maintain::Poll);
    }
}

fn to_winit_cursor(c: CursorStyle) -> winit::window::CursorIcon {
    use winit::window::CursorIcon;
    match c {
        CursorStyle::Default    => CursorIcon::Default,
        CursorStyle::Pointer    => CursorIcon::Pointer,
        CursorStyle::Text       => CursorIcon::Text,
        CursorStyle::NotAllowed => CursorIcon::NotAllowed,
        CursorStyle::Grab       => CursorIcon::Grab,
        CursorStyle::Crosshair  => CursorIcon::Crosshair,
    }
}
