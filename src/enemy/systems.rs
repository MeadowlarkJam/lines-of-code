use std::f32::consts::TAU;

use super::spawners::{spawn_shieldy, spawn_zappy, spawn_boomy};
use super::{Enemy, EnemyRoot, EnemyType};
use crate::components::{Bullet, Cannon, Projectile, WorldStats};
use crate::consts::ASSET_SPRITES_CANNON;
use crate::nodes::{spawn_zapper_node, spawn_cannon_node};
use crate::player::PlayerRoot;
use crate::{
    components::{Collider, Object, Shield, Stats, Velocity, ZapEffect, Zapper},
    consts::{
        ASSET_SPRITES_DEBRIS, ASSET_SPRITES_FORCEFIELD, ASSET_SPRITES_SHIELD, ASSET_SPRITES_ZAPPER,
    },
    events::Hit,
    nodes::{spawn_empty_node, spawn_shield_node},
    player::Player,
};
use bevy::prelude::*;
use rand::Rng;

pub fn check_enemy_death_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_stats: ResMut<WorldStats>,
    mut query: Query<(&Stats, Entity, &EnemyRoot, &Transform), With<EnemyRoot>>,
) {
    for (stats, entity, root, transform) in query.iter_mut() {
        if stats.health <= 0 {
            world_stats.enemies_alive -= 1;
            world_stats.kills += 1;
            let debris_handle = asset_server.get_handle(ASSET_SPRITES_DEBRIS);
            let shield_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_SHIELD);
            let zapper_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_ZAPPER);
            let forcefield_handle: Handle<Image> =
                asset_server.get_handle(ASSET_SPRITES_FORCEFIELD);
            let cannon_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_CANNON);
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
                EnemyType::Zappy => {
                    // Drop a zapper and 2 debris
                    let shield = spawn_zapper_node(
                        &mut commands,
                        transform.translation,
                        0.,
                        zapper_handle.clone(),
                        Zapper {
                            damage: 10,
                            fire_rate: 1.,
                            cooldown_timer: 0.,
                            range: 100.,
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
                EnemyType::Boomy => {
                    // Drop a zapper and 2 debris
                    let shield = spawn_cannon_node(
                        &mut commands,
                        transform.translation,
                        0.,
                        cannon_handle.clone(),
                        Cannon {
                            damage: 10,
                            fire_rate: 1.,
                            cooldown_timer: 0.,
                            range: 100.,
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
                },
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn shoot_zappy_enemy_system(
    mut commands: Commands,
    time: Res<Time>,
    mut event_hit: EventWriter<Hit>,
    mut zapper_query: Query<(&GlobalTransform, &mut Zapper), With<Enemy>>,
    shootable_query: Query<(&GlobalTransform, Entity, &Parent), With<Player>>,
) {
    for (zapper_transform, mut zapper_stats) in zapper_query.iter_mut() {
        if zapper_stats.cooldown_timer > 0. {
            zapper_stats.cooldown_timer -= time.delta_seconds();
        } else {
            for (shootable_transform, _shootable_entity, shootable_parent) in shootable_query.iter()
            {
                let distance = zapper_transform
                    .compute_transform()
                    .translation
                    .distance(shootable_transform.compute_transform().translation);
                // If there is a hit
                if distance < zapper_stats.range {
                    zapper_stats.cooldown_timer = zapper_stats.fire_rate;
                    event_hit.send(Hit {
                        target: shootable_parent.get(),
                        damage: zapper_stats.damage,
                    });

                    // Draw a yellow rectangle between the target and the zapper
                    let zapper_computed_transform = zapper_transform.compute_transform();

                    // Draw squares, interpolated between the two points
                    for i in 1..distance.floor() as i32 {
                        let t = i as f32 / distance.floor();
                        let x = zapper_computed_transform.translation.x
                            + (shootable_transform.compute_transform().translation.x
                                - zapper_computed_transform.translation.x)
                                * t;
                        let y = zapper_computed_transform.translation.y
                            + (shootable_transform.compute_transform().translation.y
                                - zapper_computed_transform.translation.y)
                                * t;
                        commands
                            .spawn()
                            .insert_bundle(SpriteBundle {
                                transform: Transform {
                                    translation: Vec3::new(x, y, 0.),
                                    scale: Vec3::new(2., 2., 0.),
                                    ..default()
                                },
                                sprite: Sprite {
                                    color: Color::rgb(1., 1., 0.),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(ZapEffect);
                    }

                    // Only one shot per cooldown
                    break;
                }
            }
        }
    }
}

pub fn shoot_enemy_cannon_system(
    mut commands: Commands,
    time: Res<Time>,
    mut cannon_query: Query<(&GlobalTransform, &mut Cannon), With<Enemy>>,
    shootable_query: Query<(&GlobalTransform, Entity, &Parent), With<Player>>,
) {
    // The same as player, just Player and Enemy switched
    for (cannon_transform, mut cannon_stats) in cannon_query.iter_mut() {
        if cannon_stats.cooldown_timer > 0. {
            cannon_stats.cooldown_timer -= time.delta_seconds();
        } else {
            for (shootable_transform, _shootable_entity, shootable_parent) in shootable_query.iter()
            {
                let distance = cannon_transform
                    .compute_transform()
                    .translation
                    .distance(shootable_transform.compute_transform().translation);
                // If there is a hit
                if distance < cannon_stats.range {
                    cannon_stats.cooldown_timer = cannon_stats.fire_rate;

                    let velocity_x: f32 = (shootable_transform.compute_transform().translation.x
                        - cannon_transform.compute_transform().translation.x)
                        / distance;
                    let velocity_y: f32 = (shootable_transform.compute_transform().translation.y
                        - cannon_transform.compute_transform().translation.y)
                        / distance;

                    // Shoot a bullet
                    commands
                        .spawn()
                        .insert_bundle(SpriteBundle {
                            transform: Transform {
                                translation: cannon_transform.compute_transform().translation,
                                scale: Vec3::new(2., 2., 0.),
                                ..default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1., 0., 0.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Projectile {})
                        .insert(Bullet {
                            damage: cannon_stats.damage,
                            enemy: true,
                        })
                        .insert(Velocity {
                            x: velocity_x * 2.,
                            y: velocity_y * 2.,
                            rotation: 0.,
                        });

                    // Only one shot per cooldown
                    break;
                }
            }
        }
    }
}

pub fn follow_player_in_range_system(
    player_query: Query<&Transform, With<PlayerRoot>>,
    mut enemy_query: Query<&mut Transform, (With<EnemyRoot>, Without<PlayerRoot>)>,
) {
    for player_transform in player_query.iter() {
        for mut enemy_transform in enemy_query.iter_mut() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance < 200. && distance > 8. {
                let direction = (player_transform.translation - enemy_transform.translation).normalize();
                enemy_transform.translation = enemy_transform.translation + direction * 0.6;
            } 
        }
    }
}

pub fn spawn_random_enemies_system(
    mut commands: Commands,
    mut world_stats: ResMut<WorldStats>,
    windows: Res<Windows>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<PlayerRoot>>,
) {
    // 5 enemies at max
    if world_stats.enemies_alive < world_stats.kills + 1 {
        let player_transform = player_query.single().translation;
        // Spawn a random enemy just outside of the screen
        // Due to the camera zoom, the side of the screen is actually not the side of the viewport
        let half_width = windows.get_primary().unwrap().width() as f32 / 7.;
        let half_height = windows.get_primary().unwrap().height() as f32 / 7.;
        let screen_side = rand::thread_rng().gen_range(0..4);
        let offset = 0.;
        let position = player_transform + match screen_side {
            // Left
            0 => Vec3::new(-half_width - offset, rand::thread_rng().gen_range(-half_height..half_height), 0.),
            // Right
            1 => Vec3::new(half_width + offset, rand::thread_rng().gen_range(-half_height..half_height), 0.),
            // Up
            2 => Vec3::new(rand::thread_rng().gen_range(-half_width..half_width), half_height + offset, 0.),
            // Down
            _ => Vec3::new(rand::thread_rng().gen_range(-half_width..half_width), -half_height - offset, 0.),
        };
        println!("Spawning enemy at {:?}", position);

        // Spawn a random enemy
        let enemy_type = rand::thread_rng().gen_range(0..3);
        match enemy_type {
            0 => spawn_shieldy(commands, asset_server, position),
            1 => spawn_boomy(commands, asset_server, position),
            _ => spawn_zappy(commands, asset_server, position),
        }

        world_stats.enemies_alive += 1;
    }
}

// Clean enemies if the distance is too high
pub fn clean_enemies_system(
    mut commands: Commands,
    mut world_stats: ResMut<WorldStats>,
    player_query: Query<&Transform, With<PlayerRoot>>,
    enemy_query: Query<(&Transform, Entity), With<EnemyRoot>>
) {
    for player_transform in player_query.iter() {
        for (enemy_transform, enemy_entity) in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            if distance > 5000. {
                commands.entity(enemy_entity).despawn();
                world_stats.enemies_alive -= 1;
            }
        }
    }
}