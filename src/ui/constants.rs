use bevy::prelude::Color;

// Colors
#[allow(dead_code)]
pub const COLOR_BACKGROUND_DARK: Color = Color::rgb(20.0 / 255.0, 17.0 / 255.0, 18.0 / 255.0);
pub const COLOR_BACKGROUND_MEDIUM: Color = Color::rgb(69.0 / 255.0, 61.0 / 255.0, 62.0 / 255.0);
pub const COLOR_BACKGROUND_LIGHT: Color = Color::rgb(118.0 / 255.0, 116.0 / 255.0, 117.0 / 255.0);
pub const COLOR_FOREGROUND: Color = Color::rgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0);
pub const COLOR_ACCENT: Color = Color::rgb(249.0 / 255.0, 72.0 / 255.0, 64.0 / 255.0);
pub const COLOR_TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
pub const COLOR_BUTTON_DEFAULT: Color = COLOR_BACKGROUND_MEDIUM;
pub const COLOR_BUTTON_HOVERED: Color = COLOR_BACKGROUND_LIGHT;
pub const COLOR_BUTTON_CLICKED: Color = COLOR_ACCENT;

// Font sizes
pub const FONT_SIZE_LARGE: f32 = 140.0;
pub const FONT_SIZE_MEDIUM: f32 = 100.0;
pub const FONT_SIZE_SMALL: f32 = 60.0;
