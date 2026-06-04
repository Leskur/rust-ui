# Button

Button component with variants, sizes, and states.

## Variants

```rust
use rust_ui::widget::button::{Button, ButtonVariant};

button("Primary").variant(ButtonVariant::Primary);
button("Secondary").variant(ButtonVariant::Secondary);
button("Outline").variant(ButtonVariant::Outline);
button("Danger").variant(ButtonVariant::Danger);
button("Ghost").variant(ButtonVariant::Ghost);
```

## Sizes

```rust
use rust_ui::widget::button::{ButtonSize, ButtonVariant};

button("XS").variant(ButtonVariant::Secondary).size(ButtonSize::Xs);
button("SM").variant(ButtonVariant::Secondary).size(ButtonSize::Sm);
button("MD").variant(ButtonVariant::Secondary).size(ButtonSize::Md);
button("LG").variant(ButtonVariant::Secondary).size(ButtonSize::Lg);
```

## States

```rust
button("Disabled").disabled(true);
button("Loading").loading(true);
```

## Events

```rust
button("Click me").on_click(|| {
    println!("Button clicked!");
});
```

![Button Widget](../../screenshots/button.png)
