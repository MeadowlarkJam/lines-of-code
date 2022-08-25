use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyRoot {
    pub enemy_type: EnemyType
}

#[derive(Component)]
pub enum EnemyType {
    Shieldy,
    Zappy,
    Boomy
}