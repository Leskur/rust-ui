# Input

HTML 风格的文本输入框，支持光标、选区、剪贴板、IME 和滚动。

## 基础用法

```rust
use rust_ui::widget::{input, InputSize};

input("").placeholder("输入内容...").width(300.0);
input("hello@example.com").placeholder("邮箱").width(300.0);
```

## 尺寸（高度变体）

```rust
input("").placeholder("小号").size(InputSize::Sm).width(140.0);
input("").placeholder("中号（默认）").size(InputSize::Md).width(180.0);
input("").placeholder("大号").size(InputSize::Lg).width(140.0);
```

## 可清除

```rust
input("").placeholder("搜索...").clearable().width(300.0);
```

## 密码模式

```rust
input("").placeholder("输入密码").password().width(300.0);
input("secret123").password().clearable().width(300.0);
```

## 事件

```rust
input("")
    .on_change(|text| println!("值: {}", text))
    .on_submit(|text| println!("提交: {}", text))
    .on_clear(|| println!("已清除"));
```

## 键盘快捷键

| 快捷键 | 操作 |
|--------|------|
| `←` `→` | 左右移动光标 |
| `Shift + ←` `→` | 扩展选区 |
| `Home` / `End` | 跳至行首/行尾 |
| `Shift + Home` / `End` | 扩展选区到边缘 |
| `Ctrl + A` | 全选 |
| `Ctrl + C` | 复制选区 |
| `Ctrl + X` | 剪切选区 |
| `Ctrl + V` | 粘贴剪贴板内容 |
| `Backspace` | 删除光标左侧字符 |
| `Delete` | 删除光标右侧字符 |
| `Enter` | 提交（触发 `on_submit`） |
| `Escape` | 取消 IME 预编辑或取消焦点 |

## IME 支持

完整支持中文/日文/韩文输入法。预编辑文本在提交前显示为带下划线的浅色文字。

![Input 组件](../../screenshots/input.png)
