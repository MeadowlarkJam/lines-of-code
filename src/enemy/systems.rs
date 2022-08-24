use super::{Enemy, EnemyRoot};
use crate::{
    components::{Collider, Shield, Stats, Velocity},
    consts::{ASSET_SPRITES_DEBRIS, ASSET_SPRITES_ZAPPER},
    nodes::{spawn_empty_node, spawn_shield_node},
};
use bevy::prelude::*;

pub fn _check_enemy_death_system(
    mut commands: Commands,
    mut query: Query<(&Stats, Entity), With<EnemyRoot>>,
) {
    for (stats, entity) in query.iter_mut() {
        if stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_shieldy_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(0., 0., 0.);

    let zapper_handle = asset_server.get_handle(ASSET_SPRITES_ZAPPER);
    let debris_handle = asset_server.get_handle(ASSET_SPRITES_DEBRIS);

    let root = spawn_empty_node(&mut commands, position, 0., debris_handle.clone());
    commands
        .entity(root)
        .insert(Collider)
        .insert(Enemy)
        .insert(EnemyRoot)
        .insert(Stats {
            size: 7,
            health: 10000,
        })
        .insert(Velocity {
            x: 0.,
            y: 0.,
            rotation: 0.4,
        });

    // Arms and attach them to the root
    for i in 1..=2 {
        let element_right = spawn_empty_node(
            &mut commands,
            Vec3::new(i as f32 * 8., 0., 0.),
            rand::random::<f32>() * 2. * std::f32::consts::PI,
            debris_handle.clone(),
        );
        commands
            .entity(element_right)
            .insert(Collider)
            .insert(Enemy);

        let element_left = spawn_empty_node(
            &mut commands,
            Vec3::new(-i as f32 * 8., 0., 0.),
            rand::random::<f32>() * 2. * std::f32::consts::PI,
            debris_handle.clone(),
        );
        commands.entity(element_left).insert(Collider).insert(Enemy);

        commands.entity(root).add_child(element_right);
        commands.entity(root).add_child(element_left);
    }
    let shield_right = spawn_shield_node(
        &mut commands,
        Vec3::new(16., 8., 0.),
        rand::random::<f32>() * 2. * std::f32::consts::PI,
        zapper_handle.clone(),
        Shield {
            health: 10,
            cooldown: 3.,
            cooldown_timer: 0.,
        },
    );

    commands.entity(shield_right).insert(Collider).insert(Enemy);

    let shield_left = spawn_shield_node(
        &mut commands,
        Vec3::new(-16., -8., 0.),
        rand::random::<f32>() * 2. * std::f32::consts::PI,
        zapper_handle.clone(),
        Shield {
            health: 100,
            cooldown: 3.,
            cooldown_timer: 0.,
        },
    );
    commands.entity(shield_left).insert(Collider).insert(Enemy);

    commands.entity(root).add_child(shield_right);
    commands.entity(root).add_child(shield_left);
}
