# Select

下拉选择器，支持分组标签、分隔线与可滚动菜单。

## 基础用法

```rust
use rust_ui::prelude::*;

select(vec!["Apple", "Banana", "Cherry"])
    .placeholder("Select a fruit")
    .width(180.0);
```

## 分组条目

```rust
use rust_ui::widget::SelectEntry;

select_entries(vec![
    SelectEntry::label("Fruits"),
    SelectEntry::item("Apple"),
    SelectEntry::separator(),
    SelectEntry::item("Carrot"),
])
.on_change(|label| println!("{label}"));
```

## 状态

```rust
select(vec!["A", "B"]).disabled(true);
select(vec!["A", "B"]).invalid(true);
select(vec!["A", "B"]).selected_value("A");
```
