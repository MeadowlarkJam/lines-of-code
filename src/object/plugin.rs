use super::systems::{move_objects_system, spawn_start_objects_system};
use crate::{player::PlayerSystem, schedule::GameState};
use bevy::{prelude::*, time::FixedTimestep};

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
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(move_objects_system),
        );
    }
}
