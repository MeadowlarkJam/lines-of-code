use bevy::prelude::Color;

// Player
pub const PLAYER_SPEED: f32 = 2.;

// UI
pub const COLOR_BACKGROUND_DARKEST: Color = Color::rgb(20.0 / 255.0, 17.0 / 255.0, 18.0 / 255.0);
pub const COLOR_BACKGROUND_DARK: Color = Color::rgb(33.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0);
pub const COLOR_BACKGROUND_MEDIUM: Color = Color::rgb(69.0 / 255.0, 61.0 / 255.0, 62.0 / 255.0);
pub const COLOR_BACKGROUND_LIGHT: Color = Color::rgb(118.0 / 255.0, 116.0 / 255.0, 117.0 / 255.0);
pub const COLOR_FOREGROUND: Color = Color::rgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0);
pub const COLOR_ACCENT: Color = Color::rgb(249.0 / 255.0, 72.0 / 255.0, 64.0 / 255.0);

pub const COLOR_BUTTON_DEFAULT: Color = COLOR_BACKGROUND_MEDIUM;
pub const COLOR_BUTTON_HOVERED: Color = COLOR_BACKGROUND_LIGHT;
pub const COLOR_BUTTON_CLICKED: Color = COLOR_ACCENT;

// Assets
pub const ASSET_FONTS_DEFAULT: &str = "fonts/FiraMono-Medium.ttf";
pub const ASSET_SPRITES_ZAPPER: &str = "sprites/zapper.png";
pub const ASSET_SPRITES_DEBRIS: &str = "sprites/debris.png";
pub const ASSET_SPRITES_PLAYER: &str = "sprites/player.png";
