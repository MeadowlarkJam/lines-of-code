use super::systems::{move_objects_system, spawn_start_objects_system};
use crate::player::PlayerSystem;
use bevy::{prelude::*, time::FixedTimestep};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct ObjectSystem;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_start_objects_system.label(ObjectSystem))
            .add_system_set(
                SystemSet::new()
                    .label(ObjectSystem)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                    .with_system(move_objects_system.before(PlayerSystem)),
            );
    }
}
