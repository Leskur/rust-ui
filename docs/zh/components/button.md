# Button

按钮组件，支持多种变体、尺寸和状态。

## 变体

```rust
use rust_ui::widget::button::{Button, ButtonVariant};

button("Primary").variant(ButtonVariant::Primary);
button("Secondary").variant(ButtonVariant::Secondary);
button("Outline").variant(ButtonVariant::Outline);
button("Danger").variant(ButtonVariant::Danger);
button("Ghost").variant(ButtonVariant::Ghost);
```

## 尺寸

```rust
use rust_ui::widget::button::{ButtonSize, ButtonVariant};

button("XS").variant(ButtonVariant::Secondary).size(ButtonSize::Xs);
button("SM").variant(ButtonVariant::Secondary).size(ButtonSize::Sm);
button("MD").variant(ButtonVariant::Secondary).size(ButtonSize::Md);
button("LG").variant(ButtonVariant::Secondary).size(ButtonSize::Lg);
```

## 状态

```rust
button("Disabled").disabled(true);
button("Loading").loading(true);
```

## 事件

```rust
button("点击我").on_click(|| {
    println!("按钮被点击了！");
});
```

![Button 组件](../../screenshots/button.png)
