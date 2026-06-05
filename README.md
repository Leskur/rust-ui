# rust-ui

> A beautiful, ergonomic UI library for Rust desktop applications.

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
| Verbose, unfamiliar API | Chainable builder API inspired by SwiftUI + shadcn/ui |

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

Run the interactive showcase:

```bash
cargo run -p showcase
```

## Screenshots

![Button Widget](screenshots/button.png)

![Input Widget](screenshots/input.png)

![Switch Widget](screenshots/switch.png)

## Documentation

- [Components](docs/en/components/) — Button, Input, Switch, and more
- [Guides](docs/en/guides/) — Theming, Animation, Layout

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
        ├── Button     Primary / Secondary / Danger / Ghost variants, sizes, states
        ├── Text       size, color, bold, font family
        ├── Input      HTML-like editing: cursor, selection, clipboard, IME, scroll
        ├── Switch     animated toggle
        ├── Row        horizontal flex container
        ├── Column     vertical flex container
        ├── Container  styled box with padding / border / radius
        ├── Image      local file + network URL, async loading, ObjectFit
        ├── Svg        SVG file / URL rasterized via resvg
        ├── Icon       hardcoded lucide-style vector icons
        ├── CodeBlock  dark-background monospace code display
        ├── ScrollArea scrollable container
        ├── Divider    horizontal / vertical separator
        ├── Spacer     flexible space
        ├── Stack      layered z-axis container
        ├── Sidebar    navigation panel with groups
        └── TabView    tabbed panels

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

### ✅ Phase 2 — Core components (done)
- [x] `Divider` — horizontal / vertical separator
- [x] `ScrollArea` — scrollable container
- [x] `Spacer` — flexible space
- [x] `Stack` — z-axis layered container
- [x] `Sidebar` — navigation panel with groups
- [x] `TabView` — tabbed panels
- [x] `Button` size variants: `xs / sm / md / lg`
- [x] `Image` — local + network image, async loading, ObjectFit
- [x] `Svg` — SVG file / URL rasterized via resvg
- [x] `Icon` — vector icon widget (lucide-style)
- [x] `CodeBlock` — monospace code display with line numbers

### 📦 Phase 3 — Interactive components
- [ ] `Select` — dropdown picker
- [ ] `Checkbox` — multi-select
- [ ] `Radio` — single-select group
- [ ] `Slider` — drag to set value
- [ ] `Tooltip` — hover hint
- [ ] `Dropdown Menu` — context menu
- [ ] `Dialog / Modal` — overlay with focus trap
- [ ] Event bubbling mechanism

### 📊 Phase 4 — Data & layout
- [ ] `Table` — virtualized rows for large datasets
- [ ] `Progress` — progress bar
- [ ] `Toast` — ephemeral notification
- [ ] `Accordion` — collapsible sections
- [ ] `Resizable` — drag-to-resize panels

### ✨ Phase 5 — Polish
- [ ] Theme hot-reload from TOML file
- [ ] Accessibility (ARIA-equivalent metadata)
- [ ] `cargo add rust-ui-cli` — shadcn-style component installer

## License

MIT
