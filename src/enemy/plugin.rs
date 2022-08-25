use crate::{
    despawn_recursive::despawn_entities_recursive_system, player::PlayerSystem, schedule::GameState,
};

use super::{
    systems::{
        check_enemy_death_system, enemy_bullet_collision, shoot_enemy_cannon_system,
        shoot_zappy_enemy_system, spawn_shieldy_enemy_system, spawn_zappy_enemy_system,
    },
    EnemyRoot,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct EnemySystem;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(EnemySystem)
                .with_system(spawn_shieldy_enemy_system)
                .with_system(spawn_zappy_enemy_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(EnemySystem)
                .with_system(check_enemy_death_system.after(PlayerSystem))
                .with_system(shoot_zappy_enemy_system)
                .with_system(shoot_enemy_cannon_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::MainMenu)
                .label(EnemySystem)
                .with_system(despawn_entities_recursive_system::<EnemyRoot>),
        );
    }
}
