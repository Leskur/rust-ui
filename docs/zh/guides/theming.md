# 主题定制

通过 `Theme` 系统自定义颜色、间距和排版。

## 内置主题

```rust
use rust_ui::style::Theme;

let theme = Theme::dark();   // VS Code 风格深色主题
let theme = Theme::light();  // 简洁浅色主题
```

## 主题 Token

```rust
theme.accent      // 强调色（主要操作）
theme.fg          // 主文字色
theme.fg_subtle   // 次要文字色
theme.bg          // 背景色
theme.bg_surface  // 卡片/面板背景
theme.border      // 边框颜色
theme.red         // 语义红色（错误、危险）
theme.radius_sm   // 小圆角
theme.radius_md   // 中圆角
theme.radius_lg   // 大圆角
theme.font_size_sm // 小字号
theme.font_size_md // 中字号（默认）
theme.font_size_lg // 大字号
```

## 自定义主题

```rust
use rust_ui::style::{Theme, Color};

let custom_theme = Theme {
    bg:         Color::hex("#0a0a0a"),
    bg_surface: Color::hex("#141414"),
    fg:         Color::hex("#e0e0e0"),
    fg_subtle:  Color::hex("#888888"),
    border:     Color::hex("#333333"),
    accent:     Color::hex("#ff6b6b"),
    red:        Color::hex("#ff4757"),
    radius_sm:  4.0,
    radius_md:  8.0,
    radius_lg:  12.0,
    font_size_sm: 12.0,
    font_size_md: 14.0,
    font_size_lg: 16.0,
};
```

## 单个组件样式覆盖

使用 `Style` 系统为特定组件覆盖主题默认值：

```rust
use rust_ui::style::Style;
use rust_ui::color::Color;

let custom_style = Style::new()
    .bg(Color::hex("#1e2130"))
    .radius(12.0)
    .padding(16.0)
    .border_color(Color::hex("#2a2d3e"))
    .border_width(1.0);

container(content).style(custom_style);
```
