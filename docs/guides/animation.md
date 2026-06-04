# Animation

Centralized animation scheduler — widgets never manage time.

## How It Works

The `AnimationScheduler` owns all active tweens. You trigger animations from event handlers, and the runtime ticks the scheduler once per frame.

## Trigger Animation

```rust
use rust_ui::animation::{AnimationScheduler, Easing};

// In your event handler:
scheduler.animate_to(
    "my-switch",      // widget ID
    "progress",       // property name
    1.0,              // target value
    Easing::EaseOut,  // easing function
    0.18,             // duration in seconds
);
```

## Tick Per Frame

```rust
// In your render loop / event loop:
scheduler.tick(dt);  // dt is delta time in seconds
```

## Read Current Value

```rust
// In your widget's draw() method:
let progress = scheduler.get("my-switch", "progress").unwrap_or(0.0);
```

## Easing Functions

```rust
use rust_ui::animation::Easing;

Easing::Linear
Easing::EaseIn
Easing::EaseOut
Easing::EaseInOut
```

## Example: Animated Switch

```rust
// In Switch widget's handle_event():
if clicked {
    scheduler.animate_to(id, "progress", 1.0, Easing::EaseOut, 0.18);
}

// In Switch widget's draw():
let p = scheduler.get(id, "progress").unwrap_or(0.0);
let thumb_x = lerp(start_x, end_x, p);
```

This centralized approach prevents animation stuttering — the scheduler actively advances all tweens each frame, rather than waiting for events to trigger updates.
