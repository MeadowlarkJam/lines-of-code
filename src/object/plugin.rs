use super::{
    systems::{
        bullet_collision, clean_bullets, forcefield_cooldown_system, move_objects_system,
        move_projectile, spawn_start_objects_system, velocity_dropoff_system,
    },
    Object,
};
use crate::{
    components::{Projectile, ZapEffect},
    despawn_recursive::despawn_entities_recursive_system,
    player::PlayerSystem,
    schedule::GameState,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct ObjectSystem;

pub struct ObjectPlugin;

impl Plugin for ObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .label(ObjectSystem)
                .with_system(spawn_start_objects_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(ObjectSystem)
                .before(PlayerSystem)
                .with_system(move_objects_system)
                .with_system(move_projectile)
                .with_system(velocity_dropoff_system)
                .with_system(bullet_collision)
                .with_system(forcefield_cooldown_system)
                .with_system(clean_bullets.after(bullet_collision)),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::InGame)
                .label(ObjectSystem)
                .with_system(despawn_entities_recursive_system::<Object>)
                .with_system(despawn_entities_recursive_system::<ZapEffect>)
                .with_system(despawn_entities_recursive_system::<Projectile>),
        );
    }
}
