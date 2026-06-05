# Dialog

模态对话框，带背景遮罩、Esc 关闭与焦点陷阱。

## 基础用法

```rust
use rust_ui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

let open = Rc::new(RefCell::new(false));

stack![
    main_content,
    dialog(column![
        text("Are you sure?"),
        button("Confirm").on_click({
            let o = open.clone();
            move || *o.borrow_mut() = false
        }),
    ])
    .title("Confirm")
    .shared_open(open.clone()),
];
```

## 选项

```rust
dialog(content)
    .width(420.0)
    .close_on_backdrop(true)
    .close_on_escape(true)
    .on_close(|| println!("closed"));
```
