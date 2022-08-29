use super::{
    resources::LoadingAssets,
    systems::{check_if_assets_are_loaded_system, load_assets_system},
    AudioHandles, FontHandles, SpriteHandles,
};
use crate::schedule::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct AssetSystem;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FontHandles::default())
            .insert_resource(AudioHandles::default())
            .insert_resource(SpriteHandles::default())
            .insert_resource(LoadingAssets::default())
            .add_system_set(
                SystemSet::on_enter(GameState::AssetLoading)
                    .label(AssetSystem)
                    .with_system(load_assets_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::AssetLoading)
                    .label(AssetSystem)
                    .with_system(check_if_assets_are_loaded_system),
            );
    }
}
