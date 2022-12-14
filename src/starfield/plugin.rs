use super::systems::{spawn_starfield_system, update_starfield_size_system};
use crate::schedule::GameState;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct StarfieldSystem;

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::BeforeSplashScreen)
                .label(StarfieldSystem)
                .with_system(spawn_starfield_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(StarfieldSystem)
                .with_system(update_starfield_size_system),
        );
    }
}
