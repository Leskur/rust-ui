//! wgpu + winit rendering backend for rust-ui.
//!
//! This crate implements the `rust_ui::render::Renderer` trait using
//! `wgpu` for GPU access and `vello` for 2D vector rendering.
//!
//! # Status
//! 🚧 Work in progress — stubs present, implementation pending.

pub mod renderer;
pub mod text;
pub mod window;

pub use renderer::VelloRenderer;
pub use window::{run, run_with_scheduler};
