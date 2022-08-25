use std::f32::consts::TAU;

use super::spawners::{spawn_shieldy, spawn_zappy};
use super::{Enemy, EnemyRoot, EnemyType};
use crate::components::{Bullet, Cannon, Projectile};
use crate::nodes::spawn_zapper_node;
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

pub fn check_enemy_death_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Stats, Entity, &EnemyRoot, &Transform), With<EnemyRoot>>,
) {
    for (stats, entity, root, transform) in query.iter_mut() {
        if stats.health <= 0 {
            let debris_handle = asset_server.get_handle(ASSET_SPRITES_DEBRIS);
            let shield_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_SHIELD);
            let zapper_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_ZAPPER);
            let forcefield_handle: Handle<Image> =
                asset_server.get_handle(ASSET_SPRITES_FORCEFIELD);
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
                EnemyType::Boomy => todo!(),
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

pub fn enemy_bullet_collision(
    mut commands: Commands,
    mut event_hit: EventWriter<Hit>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut forcefield_query: Query<
        (&GlobalTransform, &mut Visibility, &mut Shield),
        (With<Player>, With<Parent>),
    >,
) {
    // This is the player code, but with Enemy and Player switched

    // Check for forcefield collision
    // On forcefield collision, just do the forcefield damage here and remove the bullet
    // Forcefield radius is 18 units
    for (forcefield_transform, mut forcefield_visibility, mut forcefield_stats) in
        forcefield_query.iter_mut()
    {
        for (bullet_entity, bullet_transform, bullet_stats) in bullet_query.iter() {
            let distance = forcefield_transform
                .compute_transform()
                .translation
                .distance(bullet_transform.translation);
            if distance < 18. && forcefield_visibility.is_visible {
                forcefield_stats.health -= bullet_stats.damage;
                if forcefield_stats.health <= 0 {
                    forcefield_visibility.is_visible = false;
                }

                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }

    // If no forcefield, check for enemy collision
    // On collision, do hit event and remove bullet
    for (enemy_entity, enemy_transform) in player_query.iter() {
        for (bullet_entity, bullet_transform, bullet_stats) in bullet_query.iter() {
            let distance = enemy_transform
                .compute_transform()
                .translation
                .distance(bullet_transform.translation);
            if distance < 5. {
                event_hit.send(Hit {
                    target: enemy_entity,
                    damage: bullet_stats.damage,
                });
                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }
}

pub fn spawn_shieldy_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(0., 0., 0.);

    spawn_shieldy(commands, asset_server, position);
}

pub fn spawn_zappy_enemy_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let position = Vec3::new(100., 100., 0.);

    spawn_zappy(commands, asset_server, position);
}
