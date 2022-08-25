use super::systems::{move_objects_system, spawn_start_objects_system, velocity_dropoff_system};
use crate::{
    components::Object, despawn_recursive::despawn_entities_recursive_system, player::PlayerSystem,
    schedule::GameState,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct ObjectSystem;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(ObjectSystem)
                .with_system(spawn_start_objects_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(ObjectSystem)
                .before(PlayerSystem)
                .with_system(move_objects_system)
                .with_system(velocity_dropoff_system)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .label(ObjectSystem)
                .with_system(despawn_entities_recursive_system::<Object>),
        );
    }
}
