# rust-ui

> A beautiful, frontend-friendly UI library for Rust desktop applications.  
> 为前端开发者设计的 Rust 桌面 UI 库。

Built by a frontend developer who got tired of ugly Rust UIs.  
由一位受够了 Rust UI 丑陋外观的前端开发者构建。

---

**[中文文档 →](./README.zh.md)**

---

## Why another UI library?

| Problem with existing libs | rust-ui's answer |
|---|---|
| Styles require 20-line closures | `Style` is a value — create once, reuse everywhere |
| No CSS-like inheritance | `style.merge(other)` — later values win, like CSS specificity |
| Animation requires boilerplate | `AnimationScheduler` owns all tweens, tick once per frame |
| Renderer locked to one backend | `Renderer` trait — swap wgpu, skia, or test backends freely |
| API unfamiliar to web devs | Builder API inspired by SwiftUI + shadcn/ui naming |

## Quick start

```rust
use rust_ui::prelude::*;

let ui = column![
    text("Hello, rust-ui!").size(24.0).bold(),
    text("Subtitle").color(Color::hex("#8b8fa8")),
    button("Get started")
        .on_click(|| println!("clicked!")),
]
.spacing(12.0)
.padding(24.0);

rust_ui_wgpu::run("My App", 800, 600, ui, Theme::dark());
```

## Style system

Inspired by shadcn/ui — styles are plain values you own and compose.

```rust
use rust_ui::style::Style;
use rust_ui::color::Color;

// Define a reusable style (like a CSS class)
let card = Style::new()
    .bg(Color::hex("#1e2130"))
    .radius(12.0)
    .padding(16.0)
    .border_color(Color::hex("#2a2d3e"))
    .border_width(1.0);

// Derive a variant — merge() works like CSS specificity
let active_card = card.clone().border_color(Color::hex("#5c7cfa"));
```

## Theming

```rust
use rust_ui::style::Theme;

let theme = Theme::dark();   // VS Code-inspired dark
let theme = Theme::light();  // Clean light theme
```

## Animation

The `AnimationScheduler` owns all active tweens. Widgets never manage time.

```rust
// Trigger from event handler:
scheduler.animate_to("my-switch", "progress", 1.0, Easing::EaseOut, 0.18);

// Runtime calls once per frame:
scheduler.tick(dt);

// Widget reads current value:
let p = scheduler.get("my-switch", "progress").unwrap_or(0.0);
```

## Architecture

```
rust-ui               core library — zero GPU dependencies
  ├── color           Color with hex() / lerp()
  ├── style           Style (composable), Theme (dark / light)
  ├── render          Renderer trait — backend contract
  ├── event           Input event types
  ├── layout          Flexbox engine (taffy)
  ├── animation       AnimationScheduler + Easing
  └── widget
        ├── Button    Primary / Secondary / Danger / Ghost variants
        ├── Text      size, color, bold, font family
        ├── Input     text input with placeholder + cursor
        ├── Switch    animated toggle
        ├── Row       horizontal flex container
        ├── Column    vertical flex container
        └── Container styled box with padding / border / radius

rust-ui-wgpu          wgpu + vello + winit rendering backend
  ├── renderer        VelloRenderer implements Renderer trait
  ├── text            cosmic-text font shaping
  └── window          winit event loop + frame pump
```

## Roadmap

Inspired by [shadcn/ui](https://ui.shadcn.com/docs/components) component set.

### ✅ Phase 1 — Foundation (done)
- [x] Color, Style, Theme
- [x] Renderer trait + wgpu/vello backend
- [x] AnimationScheduler
- [x] Button, Text, Input, Switch, Row, Column, Container
- [x] counter + gallery examples running

### 🔨 Phase 2 — Core components
- [ ] `Badge` — small status label
- [ ] `Separator` — horizontal / vertical divider
- [ ] `Spinner` — loading animation (validates AnimationScheduler)
- [ ] `Avatar` — circular image / initials placeholder
- [ ] `Button` size variants: `xs / sm / md / lg`

### 📦 Phase 3 — Interactive components
- [ ] `Select` — dropdown picker
- [ ] `Checkbox` — multi-select
- [ ] `Radio` — single-select group
- [ ] `Slider` — drag to set value
- [ ] `Tooltip` — hover hint
- [ ] `Dropdown Menu` — context menu
- [ ] `Dialog / Modal` — overlay with focus trap
- [ ] `Tabs` — tabbed panels

### 📊 Phase 4 — Data & layout
- [ ] `Table` — virtualized rows for large datasets
- [ ] `ScrollView` — virtualized scroll list
- [ ] `Progress` — progress bar
- [ ] `Toast` — ephemeral notification
- [ ] `Accordion` — collapsible sections
- [ ] `Sidebar` — navigation panel
- [ ] `Resizable` — drag-to-resize panels

### ✨ Phase 5 — Polish
- [ ] Theme hot-reload from TOML file
- [ ] Accessibility (ARIA-equivalent metadata)
- [ ] SVG icon support
- [ ] `cargo add rust-ui-cli` — shadcn-style component installer

## License

MIT
