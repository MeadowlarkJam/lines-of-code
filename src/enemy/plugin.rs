use crate::{
    despawn_recursive::despawn_entities_recursive_system, player::PlayerSystem, schedule::GameState,
};

use super::{
    systems::{
        check_enemy_death_system, clean_enemies_system, follow_player_in_range_system,
        shoot_enemy_cannon_system, shoot_zappy_enemy_system, spawn_random_enemies_system,
    },
    EnemyKilled, EnemyRoot, EnemySpawned,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct EnemySystem;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyKilled>()
            .add_event::<EnemySpawned>()
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(EnemySystem)
                    .with_system(check_enemy_death_system.after(PlayerSystem))
                    .with_system(shoot_zappy_enemy_system)
                    .with_system(shoot_enemy_cannon_system)
                    .with_system(follow_player_in_range_system)
                    .with_system(clean_enemies_system.before(spawn_random_enemies_system))
                    .with_system(spawn_random_enemies_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterInGame)
                    .label(EnemySystem)
                    .with_system(despawn_entities_recursive_system::<EnemyRoot>),
            );
    }
}
