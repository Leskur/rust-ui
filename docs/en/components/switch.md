# Switch

Animated toggle switch with on/off state.

## Basic Usage

```rust
use rust_ui::widget::switch;

switch("System Proxy", true);
switch("Dark Mode", true);
switch("Notifications", false);
```

## Size

```rust
use rust_ui::widget::{switch, SwitchSize};

switch("Small", true).size(SwitchSize::Sm);
switch("Medium (default)", true).size(SwitchSize::Md);
switch("Large", true).size(SwitchSize::Lg);
```

## Disabled

```rust
switch("Disabled", true).disabled(true);
```

## Events

```rust
switch("System Proxy", proxy_enabled)
    .on_change(|on| println!("Proxy: {}", on));
```

![Switch Widget](../../screenshots/switch.png)
