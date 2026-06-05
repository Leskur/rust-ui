# Toast

堆叠式通知（Sonner 风格），右下角显示，支持自动消失与手动关闭。

## 挂载 Toaster

```rust
use rust_ui::prelude::*;

let store = toast_store();

stack![
    app_content,
    toaster(store.clone()),
];
```

## 触发通知

```rust
store.borrow_mut().show("Event created");
store.borrow_mut().success("Saved");
store.borrow_mut().error("Something went wrong");

store.borrow_mut().message(
    "Scheduled",
    "Friday at 5:57 PM",
);
```

## 自定义

```rust
ToastItem::new("Title")
    .description("Details")
    .variant(ToastVariant::Warning)
    .duration_ms(8000); // 0 = 不自动关闭
```
