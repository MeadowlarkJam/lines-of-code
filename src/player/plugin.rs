use super::systems::{
    check_attachment_system, check_hits_system, check_player_death_system, move_player_system,
    remove_zap_effect_system, shoot_player_zapper_system, spawn_player_system,
};
use crate::object::ObjectSystem;
use bevy::{prelude::*, time::FixedTimestep};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct PlayerSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player_system.label(PlayerSystem))
            .add_system_set(
                SystemSet::new()
                    .label(PlayerSystem)
                    .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                    .with_system(move_player_system.before(check_attachment_system))
                    .with_system(check_attachment_system.after(ObjectSystem)),
            )
            .add_system_set(
                SystemSet::new()
                    .label(PlayerSystem)
                    .with_run_criteria(FixedTimestep::step(1.0 / 4.0))
                    .with_system(remove_zap_effect_system),
            )
            .add_system_set(
                SystemSet::new()
                    .label(PlayerSystem)
                    .with_system(shoot_player_zapper_system)
                    .with_system(check_hits_system.after(shoot_player_zapper_system))
                    .with_system(check_player_death_system.after(check_hits_system)),
            );
    }
}
