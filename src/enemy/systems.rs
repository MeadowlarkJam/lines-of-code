use std::f32::consts::TAU;

use super::{Enemy, EnemyRoot, EnemyType};
use crate::{
    components::{Collider, Shield, Stats, Velocity, Object},
    consts::{ASSET_SPRITES_DEBRIS, ASSET_SPRITES_ZAPPER, ASSET_SPRITES_SHIELD, ASSET_SPRITES_FORCEFIELD},
    nodes::{spawn_empty_node, spawn_shield_node},
};
use bevy::prelude::*;

pub fn check_enemy_death_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Stats, Entity, &EnemyRoot, &Transform), With<EnemyRoot>>,
) {
    for (stats, entity, root, transform) in query.iter_mut() {
        if stats.health <= 0 {
            let debris_handle = asset_server.get_handle(ASSET_SPRITES_DEBRIS);
            let shield_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_SHIELD);
            let forcefield_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_FORCEFIELD);
            match root.enemy_type {
                EnemyType::Shieldy => {
                    // Drop a shield and 2 debris
                    let shield = spawn_shield_node(
                        &mut commands,
                        transform.translation,
                        0.,
                        shield_handle.clone(),
                        forcefield_handle.clone(),
                        Shield {
                            health: 100,
                            cooldown: 3.,
                            cooldown_timer: 0.,
                        },
                    );

                    commands
                        .entity(shield)
                        .insert(Velocity {
                            x: rand::random::<f32>() * 2. - 1.,
                            y: rand::random::<f32>() * 2. - 1.,
                            rotation: rand::random::<f32>() * 0.1,
                        })
                        .insert(Object {});

                    for _ in 0..2 {
                        // Spawn debris
                        let debris = spawn_empty_node(
                            &mut commands,
                            transform.translation,
                            rand::random::<f32>() * TAU,
                            debris_handle.clone(),
                        );

                        commands.entity(debris).insert(Object {}).insert(Velocity {
                            x: rand::random::<f32>() - 0.5,
                            y: rand::random::<f32>() - 0.5,
                            rotation: rand::random::<f32>() * 0.2,
                        });
                    }
                }
                EnemyType::Zappy => todo!(),
                EnemyType::Boomy => todo!(),
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_shieldy_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(0., 0., 0.);

    let debris_handle = asset_server.get_handle(ASSET_SPRITES_DEBRIS);
    let shield_handle = asset_server.get_handle(ASSET_SPRITES_SHIELD);
    let forcefield_handle = asset_server.get_handle(ASSET_SPRITES_FORCEFIELD);

    let root = spawn_empty_node(&mut commands, position, 0., debris_handle.clone());
    commands
        .entity(root)
        .insert(Collider)
        .insert(Enemy)
        .insert(EnemyRoot {
            enemy_type: EnemyType::Shieldy,
        })
        .insert(Stats {
            size: 7,
            health: 100,
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
        shield_handle.clone(),
        forcefield_handle.clone(),
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
        shield_handle.clone(),
        forcefield_handle.clone(),
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
