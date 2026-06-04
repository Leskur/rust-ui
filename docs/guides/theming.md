# Theming

Customize colors, spacing, and typography with the `Theme` system.

## Built-in Themes

```rust
use rust_ui::style::Theme;

let theme = Theme::dark();   // VS Code-inspired dark theme
let theme = Theme::light();  // Clean light theme
```

## Theme Tokens

```rust
theme.accent      // Accent color (primary actions)
theme.fg          // Primary text color
theme.fg_subtle   // Secondary text color
theme.bg          // Background color
theme.bg_surface  // Card/panel background
theme.border      // Border color
theme.red         // Semantic red (errors, danger)
theme.radius_sm   // Small border radius
theme.radius_md   // Medium border radius
theme.radius_lg   // Large border radius
theme.font_size_sm // Small font size
theme.font_size_md // Medium font size (default)
theme.font_size_lg // Large font size
```

## Custom Theme

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

## Per-Widget Style Override

Use the `Style` system to override theme defaults for specific widgets:

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
