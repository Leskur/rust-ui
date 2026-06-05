# 动画系统

集中式动画调度器 — Widget 本身不管理时间。

## 工作原理

`AnimationScheduler` 拥有所有活跃的补间动画。你在事件处理器中触发动画，运行时每帧 tick 一次调度器。

## 触发动画

```rust
use rust_ui::animation::{AnimationScheduler, Easing};

// 在事件处理器中：
scheduler.animate_to(
    "my-switch",      // widget ID
    "progress",       // 属性名
    1.0,              // 目标值
    Easing::EaseOut,  // 缓动函数
    0.18,             // 持续时间（秒）
);
```

## 每帧 Tick

```rust
// 在渲染循环 / 事件循环中：
scheduler.tick(dt);  // dt 是 delta time（秒）
```

## 读取当前值

```rust
// 在 widget 的 draw() 方法中：
let progress = scheduler.get("my-switch", "progress").unwrap_or(0.0);
```

## 缓动函数

```rust
use rust_ui::animation::Easing;

Easing::Linear
Easing::EaseIn
Easing::EaseOut
Easing::EaseInOut
```

## 示例：动画开关

```rust
// 在 Switch widget 的 handle_event() 中：
if clicked {
    scheduler.animate_to(id, "progress", 1.0, Easing::EaseOut, 0.18);
}

// 在 Switch widget 的 draw() 中：
let p = scheduler.get(id, "progress").unwrap_or(0.0);
let thumb_x = lerp(start_x, end_x, p);
```

这种集中式方法避免了动画卡顿 — 调度器每帧主动推进所有补间，而不是被动等待事件触发更新。
