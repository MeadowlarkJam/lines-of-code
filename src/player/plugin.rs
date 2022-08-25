use super::{
    systems::{
        check_attachment_system, check_hits_system, check_player_death_system, move_player_system,
        player_bullet_collision, remove_zap_effect_system, shoot_player_cannon_system,
        shoot_player_zapper_system, spawn_player_system,
    },
    PlayerRoot,
};
use crate::{
    despawn_recursive::despawn_entities_recursive_system, object::ObjectSystem, schedule::GameState,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct PlayerSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(PlayerSystem)
                .with_system(spawn_player_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(PlayerSystem)
                .with_system(move_player_system.before(check_attachment_system))
                .with_system(check_attachment_system.after(ObjectSystem)),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(PlayerSystem)
                .with_system(remove_zap_effect_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(PlayerSystem)
                .with_system(shoot_player_zapper_system)
                .with_system(shoot_player_cannon_system)
                .with_system(check_hits_system.after(shoot_player_zapper_system))
                .with_system(check_player_death_system.after(check_hits_system)),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .label(PlayerSystem)
                .with_system(despawn_entities_recursive_system::<PlayerRoot>),
        );
    }
}
