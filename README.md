# rust-ui

> A beautiful, frontend-friendly UI library for Rust desktop applications.

Built by a frontend developer who got tired of ugly Rust UIs.

## Why another UI library?

| Problem with existing libs | rust-ui's answer |
|---|---|
| Styles require 20-line closures | `Style` is a value — create once, reuse everywhere |
| No CSS-like inheritance | `style.merge(other)` — later values win, like CSS specificity |
| Animation requires boilerplate | `AnimationScheduler` owns all tweens, tick once per frame |
| Renderer locked to one backend | `Renderer` trait — swap wgpu, skia, or test backends freely |
| API unfamiliar to web devs | Builder API similar to SwiftUI + Tailwind naming |

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
```

## Style system

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

// Derive a variant
let active_card = card.clone().border_color(Color::hex("#5c7cfa"));
```

## Theming

```rust
use rust_ui::style::Theme;

let theme = Theme::dark();   // VS Code-inspired dark
let theme = Theme::light();  // Clean light theme
// Coming: Theme::from_file("theme.toml")
```

## Animation

```rust
// In your event handler:
scheduler.animate_to("my-switch", "progress", 1.0, Easing::EaseOut, 0.18);

// In your frame loop:
scheduler.tick(dt);  // advances all active animations

// In your widget draw:
let p = scheduler.get("my-switch", "progress").unwrap_or(0.0);
```

## Architecture

```
rust-ui          — core library (zero GPU deps)
  ├── color      — Color type with hex/lerp helpers
  ├── style      — Style, Theme, Edges, Corners
  ├── render     — Renderer trait (backend contract)
  ├── event      — Input event types
  ├── layout     — Flexbox via taffy
  ├── animation  — AnimationScheduler
  └── widget     — Button, Text, Input, Switch, Row, Column, Container

rust-ui-wgpu     — wgpu + vello rendering backend (🚧 WIP)
  ├── renderer   — WgpuRenderer implements Renderer
  └── window     — winit event loop + frame pump
```

## Roadmap

### Phase 1 — Core (current)
- [x] Color system
- [x] Style / Theme
- [x] Renderer trait
- [x] Event system
- [x] Animation scheduler
- [x] Layout (taffy wrapper)
- [x] Button, Text, Input, Switch, Row, Column, Container

### Phase 2 — Backend
- [ ] wgpu + vello renderer
- [ ] winit window loop
- [ ] Text rendering via cosmic-text
- [ ] Working counter + gallery examples

### Phase 3 — More widgets
- [ ] Checkbox, Radio
- [ ] Select / Dropdown
- [ ] Slider
- [ ] ScrollView (virtualized)
- [ ] Modal / Overlay
- [ ] Tooltip
- [ ] Badge / Tag

### Phase 4 — Polish
- [ ] Hot-reload themes
- [ ] Accessibility (a11y)
- [ ] SVG icon support
- [ ] Table (virtualized rows)

## License

MIT
