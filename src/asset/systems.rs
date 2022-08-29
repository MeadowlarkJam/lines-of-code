use super::{
    constants::{
        ASSET_AUDIO_DEATH, ASSET_AUDIO_EXPLOSION, ASSET_AUDIO_HIT, ASSET_AUDIO_INTRO,
        ASSET_AUDIO_LASER, ASSET_AUDIO_MUSIC, ASSET_FONTS_DEFAULT, ASSET_SPRITES_CANNON,
        ASSET_SPRITES_DEBRIS, ASSET_SPRITES_FORCEFIELD, ASSET_SPRITES_PLAYER, ASSET_SPRITES_SHIELD,
        ASSET_SPRITES_ZAPPER,
    },
    resources::LoadingAssets,
    AudioHandles, FontHandles, SpriteHandles,
};
use crate::schedule::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;

pub fn load_assets_system(
    asset_server: Res<AssetServer>,
    mut font_handles: ResMut<FontHandles>,
    mut sprite_handles: ResMut<SpriteHandles>,
    mut audio_handles: ResMut<AudioHandles>,
    mut loading_assets: ResMut<LoadingAssets>,
) {
    // Fonts
    font_handles.default = asset_server.load(ASSET_FONTS_DEFAULT);

    // Sprites
    sprite_handles.debris = asset_server.load(ASSET_SPRITES_DEBRIS);
    sprite_handles.zapper = asset_server.load(ASSET_SPRITES_ZAPPER);
    sprite_handles.player = asset_server.load(ASSET_SPRITES_PLAYER);
    sprite_handles.shield = asset_server.load(ASSET_SPRITES_SHIELD);
    sprite_handles.forcefield = asset_server.load(ASSET_SPRITES_FORCEFIELD);
    sprite_handles.cannon = asset_server.load(ASSET_SPRITES_CANNON);

    // Audio
    audio_handles.death = asset_server.load(ASSET_AUDIO_DEATH);
    audio_handles.hit = asset_server.load(ASSET_AUDIO_HIT);
    audio_handles.laser = asset_server.load(ASSET_AUDIO_LASER);
    audio_handles.explosion = asset_server.load(ASSET_AUDIO_EXPLOSION);
    audio_handles.intro = asset_server.load(ASSET_AUDIO_INTRO);
    audio_handles.music = asset_server.load(ASSET_AUDIO_MUSIC);

    // Add all asset handles to the `loading_assets` collection to keep track
    // of their loading state in `check_if_assets_are_loaded`.
    loading_assets.extend(vec![
        // Fonts
        font_handles.default.clone_untyped(),
        // Sprites
        sprite_handles.debris.clone_untyped(),
        sprite_handles.zapper.clone_untyped(),
        sprite_handles.player.clone_untyped(),
        sprite_handles.shield.clone_untyped(),
        sprite_handles.forcefield.clone_untyped(),
        sprite_handles.cannon.clone_untyped(),
        // Audio
        audio_handles.death.clone_untyped(),
        audio_handles.hit.clone_untyped(),
        audio_handles.laser.clone_untyped(),
        audio_handles.explosion.clone_untyped(),
        audio_handles.intro.clone_untyped(),
        audio_handles.music.clone_untyped(),
    ]);
}

pub fn check_if_assets_are_loaded_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    loading_assets: Res<LoadingAssets>,
    mut game_state: ResMut<State<GameState>>,
) {
    match asset_server.get_group_load_state(loading_assets.iter().map(|h| h.id)) {
        LoadState::Loaded => {
            commands.remove_resource::<LoadingAssets>();
            game_state.set(GameState::BeforeSplashScreen).unwrap();
        }
        LoadState::Failed => panic!("Failed to load assets"),
        _ => {}
    }
}
