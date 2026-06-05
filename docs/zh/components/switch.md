# Switch

带动画的开关切换组件。

## 基础用法

```rust
use rust_ui::widget::switch;

switch("系统代理", true);
switch("深色模式", true);
switch("通知", false);
```

## 尺寸

```rust
use rust_ui::widget::{switch, SwitchSize};

switch("小号", true).size(SwitchSize::Sm);
switch("中号（默认）", true).size(SwitchSize::Md);
switch("大号", true).size(SwitchSize::Lg);
```

## 禁用状态

```rust
switch("禁用", true).disabled(true);
```

## 事件

```rust
switch("系统代理", proxy_enabled)
    .on_change(|on| println!("代理: {}", on));
```

![Switch 组件](../../screenshots/switch.png)
