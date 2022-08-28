use crate::schedule::GameState;
use bevy::prelude::*;

use super::systems::spawn_starfield_system;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct StarfieldSystem;

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.add_startup_system(spawn_starfield_system.label(StarfieldSystem))
            .add_system_set(SystemSet::on_update(GameState::InGame).label(StarfieldSystem));
    }
}
