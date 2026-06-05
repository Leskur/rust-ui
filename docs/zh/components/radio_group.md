# Radio Group

单选组，同一组内只能选中一项。

## 基础用法

```rust
use rust_ui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

let selected = Rc::new(RefCell::new("comfortable".to_string()));

radio_group(
    vec![
        ("default", "Default"),
        ("comfortable", "Comfortable"),
        ("compact", "Compact"),
    ],
    selected.clone(),
)
.on_change(|value| println!("{value}"));
```

## 禁用选项

```rust
use rust_ui::widget::RadioGroupOption;

radio_group_options(
    vec![
        RadioGroupOption::new("free", "Free"),
        RadioGroupOption::new("pro", "Pro").disabled(true),
    ],
    selected,
);
```

## 键盘

聚焦组后使用 ↑ ↓ 切换焦点，Space / Enter 选中。
