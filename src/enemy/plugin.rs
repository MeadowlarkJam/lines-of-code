use super::systems::spawn_shieldy_enemy_system;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct EnemySystem;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_shieldy_enemy_system.label(EnemySystem));
    }
}
