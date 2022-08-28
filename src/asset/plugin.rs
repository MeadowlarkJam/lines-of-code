use super::systems::{
    load_ingame_assets_system, load_splash_sound, load_ui_assets_system, play_sounds,
};
use crate::schedule::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct AssetSystem;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::SplashScreen)
                .label(AssetSystem)
                .with_system(load_ui_assets_system)
                .with_system(load_splash_sound),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(AssetSystem)
                .with_system(load_ingame_assets_system),
        )
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(play_sounds));
    }
}
