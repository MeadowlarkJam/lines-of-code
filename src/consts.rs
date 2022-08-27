use bevy::prelude::Color;

// Player
pub const PLAYER_SPEED: f32 = 1.;

// UI
pub const COLOR_BACKGROUND_DARK: Color = Color::rgb(20.0 / 255.0, 17.0 / 255.0, 18.0 / 255.0);
pub const COLOR_BACKGROUND_MEDIUM: Color = Color::rgb(69.0 / 255.0, 61.0 / 255.0, 62.0 / 255.0);
pub const COLOR_BACKGROUND_LIGHT: Color = Color::rgb(118.0 / 255.0, 116.0 / 255.0, 117.0 / 255.0);
pub const COLOR_FOREGROUND: Color = Color::rgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0);
pub const COLOR_ACCENT: Color = Color::rgb(249.0 / 255.0, 72.0 / 255.0, 64.0 / 255.0);
pub const COLOR_TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);

pub const COLOR_BUTTON_DEFAULT: Color = COLOR_BACKGROUND_MEDIUM;
pub const COLOR_BUTTON_HOVERED: Color = COLOR_BACKGROUND_LIGHT;
pub const COLOR_BUTTON_CLICKED: Color = COLOR_ACCENT;

// Assets
pub const ASSET_FONTS_DEFAULT: &str = "fonts/FiraMono-Medium.ttf";
pub const ASSET_SPRITES_ZAPPER: &str = "sprites/zapper.png";
pub const ASSET_SPRITES_DEBRIS: &str = "sprites/debris.png";
pub const ASSET_SPRITES_PLAYER: &str = "sprites/player.png";
pub const ASSET_SPRITES_SHIELD: &str = "sprites/shield.png";
pub const ASSET_SPRITES_FORCEFIELD: &str = "sprites/forcefield.png";
pub const ASSET_SPRITES_CANNON: &str = "sprites/cannon.png";

pub const ASSET_AUDIO_DEATH: &str = "sounds/deathsound.wav";
pub const ASSET_AUDIO_LASER: &str = "sounds/lasershot1.wav";
pub const ASSET_AUDIO_EXPLOSION: &str = "sounds/explosion.wav";
pub const ASSET_AUDIO_HIT: &str = "sounds/modhit.wav";
pub const ASSET_AUDIO_LOAD: &str = "sounds/spaceIntro.ogg";
