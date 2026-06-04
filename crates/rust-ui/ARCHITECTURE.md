# rust-ui Architecture

## Design principles

1. **Renderer-agnostic core** — `crates/rust-ui` has zero GPU dependencies.
   All drawing goes through the `Renderer` trait. Backends live in separate crates.

2. **Style is a value** — `Style` is a plain `struct` you can clone, store in
   a variable, and merge. No lifetime-bound closures.

3. **Animation is centralised** — `AnimationScheduler` owns all active tweens.
   Widgets declare *what* to animate; the scheduler handles *when* and *how much*.

4. **Widgets are values** — Every widget is a Rust struct implementing `Widget`.
   No `Box<dyn Any>` downcasting required at the call site.

## Data flow

```
Platform input (winit)
        │
        ▼
  Event (mouse, keyboard…)
        │
        ▼
  Widget tree  ──handle_event──►  state mutation + AnimationScheduler.animate_to()
        │
  AnimationScheduler.tick(dt)
        │
        ▼
  Widget tree  ──draw()──►  Renderer calls (fill_rect, draw_text…)
        │
        ▼
  WgpuRenderer  ──►  vello Scene  ──►  wgpu  ──►  GPU  ──►  screen
```

## Module responsibilities

| Module | Responsibility |
|--------|---------------|
| `color` | RGBA color with CSS hex parsing, lerp |
| `style` | `Style` (composable box model), `Theme` (semantic tokens) |
| `render` | `Renderer` trait + `Rect`, `Point`, `TextOptions` |
| `event` | `Event` enum, `EventStatus`, key/mouse types |
| `layout` | Thin wrapper around `taffy` Flexbox engine |
| `animation` | `AnimationScheduler`, `Animation`, `Easing` |
| `widget` | All built-in widgets implementing `Widget` trait |

## Component status

| Widget | File | Status |
|--------|------|--------|
| Button | `widget/button.rs` | ✅ Primary/Secondary/Danger/Ghost |
| Text | `widget/text.rs` | ✅ size, color, bold |
| Input | `widget/input.rs` | ✅ placeholder, cursor, keyboard |
| Switch | `widget/switch.rs` | ✅ animated thumb |
| Row | `widget/row.rs` | ✅ horizontal flex |
| Column | `widget/column.rs` | ✅ vertical flex |
| Container | `widget/container.rs` | ✅ bg, radius, border, padding |
| Badge | — | 📋 Phase 2 |
| Separator | — | 📋 Phase 2 |
| Spinner | — | 📋 Phase 2 |
| Select | — | 📋 Phase 3 |
| Checkbox | — | 📋 Phase 3 |
| Slider | — | 📋 Phase 3 |
| Dialog | — | 📋 Phase 3 |
| Tabs | — | 📋 Phase 3 |
| Table | — | 📋 Phase 4 |

## Adding a new widget

1. Create `src/widget/my_widget.rs`
2. Implement `Widget` — at minimum `id()` and `draw()`
3. Add a shorthand `pub fn my_widget(…) -> MyWidget` constructor
4. Re-export from `src/widget/mod.rs`
5. Add to `prelude` if it's a commonly used widget
6. Add a section in `examples/gallery/src/main.rs`

## Style merge semantics

`Style::merge(self, other)` follows CSS specificity:
- Fields set in `other` (`Some(...)`) override fields in `self`
- Fields not set in `other` (`None`) keep `self`'s value

```rust
let base  = Style::new().bg(BLACK).radius(8.0).padding(16.0);
let hover = Style::new().bg(BLUE);             // only overrides bg

let result = base.merge(&hover);
// result.background == Some(BLUE)   ← from hover
// result.border.radius == 8.0       ← kept from base
// result.padding == 16.0            ← kept from base
```

## Animation design

The key insight: **widgets don't own time**.

```
❌ Widget stores `progress: f32` and steps it in draw() — no consistent frame rate
✅ Widget calls scheduler.animate_to(id, prop, target) — scheduler handles timing
```

Every frame the runtime calls `scheduler.tick(dt)` before drawing.
Widgets read current values via `scheduler.get(id, prop)` in their `draw_with_anim()` method.

Active animations are removed from the scheduler when complete, so `scheduler.has_active()`
can be used to decide whether to request a new frame (avoids busy-looping at 60fps when idle).
