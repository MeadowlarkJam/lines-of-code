use std::time::Duration;

use super::{
    systems::{
        check_attachment_system, check_hits_system, check_player_death_system, move_player_system,
        remove_zap_effect_system, reset_sprite_tint_system, rotate_player_system,
        shoot_player_cannon_system, shoot_player_zapper_system, spawn_player_system,
        update_player_history_system, update_player_properties_system, explode_player_system,
    },
    PlayerHistory, PlayerRoot, PlayerSizeIncreased,
};
use crate::{
    despawn_recursive::despawn_entities_recursive_system, enemy::systems::shoot_zappy_enemy_system,
    object::{ObjectSystem, move_objects_system}, schedule::GameState,
};
use bevy::{prelude::*, time::FixedTimestep};

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct PlayerSystem;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerSizeIncreased>()
            .insert_resource(PlayerHistory {
                target_position: Vec3::ZERO,
                new_position: Vec3::ZERO,
                timer: Timer::new(Duration::from_secs_f32(0.2), true),
            })
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeInGame)
                    .label(PlayerSystem)
                    .with_system(spawn_player_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(PlayerSystem)
                    .with_system(move_player_system.before(rotate_player_system))
                    .with_system(rotate_player_system.before(check_attachment_system))
                    .with_system(check_attachment_system.after(ObjectSystem))
                    .with_system(update_player_properties_system.after(check_attachment_system))
                    .with_system(remove_zap_effect_system)
                    .with_system(shoot_player_zapper_system)
                    .with_system(shoot_player_cannon_system)
                    .with_system(
                        check_hits_system
                            .after(shoot_player_zapper_system)
                            .after(shoot_player_cannon_system),
                    )
                    .with_system(check_player_death_system.after(check_hits_system))
                    .with_system(update_player_history_system.before(shoot_zappy_enemy_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterInGame)
                    .label(PlayerSystem)
                    .with_system(despawn_entities_recursive_system::<PlayerRoot>),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::AfterInGame)
                .with_system(explode_player_system)
            )
            .add_system_set(
                SystemSet::on_update(GameState::AfterInGame)
                .with_system(move_objects_system)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0 / 4.))
                    .with_system(reset_sprite_tint_system),
            );
    }
}
