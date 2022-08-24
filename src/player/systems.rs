use super::{Player, PlayerRoot};
use crate::{
    components::{Collider, Object, Stats, ZapEffect, Zapper},
    enemy::{Enemy, EnemyRoot},
    events::Hit,
};
use bevy::prelude::*;

const PLAYER_SPEED: f32 = 2.;

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle = asset_server.get_handle("player.png");
    // Create a player that is on top of the root. This makes sure that we only need to attach to other non-root blocks and can query for the root-transform later on
    let player_root_entity = commands
        .spawn()
        .insert(Collider)
        .insert(Player)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            texture: player_handle.clone(),
            ..default()
        })
        .id();

    // Init the player entity
    commands
        .spawn()
        .insert(Collider)
        .insert(Player)
        .insert(PlayerRoot)
        .insert(Stats {
            size: 1,
            health: 100,
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            texture: player_handle.clone(),
            ..default()
        })
        .add_child(player_root_entity);
}

// Systems can query data in an SQL-like fashion
//
// Update the player position
pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<PlayerRoot>>,
) {
    // The player's movement directions
    let mut movement_x = 0.;
    let mut movement_y = 0.;

    let mut player_transform = query.single_mut();

    // Add the different directions. This way pressing left and right cancels out
    if keyboard_input.pressed(KeyCode::Left) {
        movement_x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        movement_x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        movement_y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        movement_y += 1.;
    }

    // Move the player and clamp it to the screen
    player_transform.translation.x = player_transform.translation.x + movement_x * PLAYER_SPEED;
    player_transform.translation.y = player_transform.translation.y + movement_y * PLAYER_SPEED;
}

pub fn shoot_player_zapper_system(
    mut commands: Commands,
    time: Res<Time>,
    mut event_hit: EventWriter<Hit>,
    mut zapper_query: Query<(&GlobalTransform, &mut Zapper), With<Player>>,
    shootable_query: Query<(&Transform, Entity, &Parent), With<Enemy>>,
) {
    for (zapper_transform, mut zapper_stats) in zapper_query.iter_mut() {
        if zapper_stats.cooldown_timer > 0. {
            zapper_stats.cooldown_timer -= time.delta_seconds();
        } else {
            for (shootable_transform, shootable_entity, shootable_parent) in shootable_query.iter()
            {
                let distance = zapper_transform
                    .compute_transform()
                    .translation
                    .distance(shootable_transform.translation);
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
                            + (shootable_transform.translation.x
                                - zapper_computed_transform.translation.x)
                                * t;
                        let y = zapper_computed_transform.translation.y
                            + (shootable_transform.translation.y
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

pub fn check_hits_system(
    mut commands: Commands,
    mut event_hit: EventReader<Hit>,
    mut player_query: Query<(&mut Stats, Entity), (With<PlayerRoot>, Without<EnemyRoot>)>,
    mut enemy_query: Query<(&mut Stats, Entity), (With<EnemyRoot>, Without<PlayerRoot>)>,
) {
    let (mut player_stats, player_entity) = player_query.single_mut();
    for hit in event_hit.iter() {
        if hit.target == player_entity {
            player_stats.health -= hit.damage;
        } else {
            match enemy_query.get_mut(hit.target) {
                Ok((mut enemy_stats, _)) => {
                    enemy_stats.health -= hit.damage;
                }
                Err(_) => {}
            }
        }
    }
}

pub fn remove_zap_effect_system(mut commands: Commands, query: Query<Entity, With<ZapEffect>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn check_attachment_system(
    mut commands: Commands,
    player_query: Query<&mut GlobalTransform, (With<Player>, Without<PlayerRoot>)>,
    mut player_root_query: Query<(Entity, &Transform, &mut Stats), With<PlayerRoot>>,
    mut attachable_query: Query<
        (Entity, &mut Transform),
        (With<Object>, Without<Player>, Without<PlayerRoot>),
    >,
) {
    // We need the transform of the root, since everything is relative to it and when adding children we need to revert it first
    let (root_entity, root_transform, mut root_stats) = player_root_query.get_single_mut().unwrap();

    for player_transform in player_query.iter() {
        for (attachable_entity, mut attachable_transform) in attachable_query.iter_mut() {
            let distance = player_transform
                .compute_transform()
                .translation
                .distance(attachable_transform.translation);
            // TODO: More exact collision detection
            if distance
                < ((player_transform.compute_transform().scale.x + attachable_transform.scale.x)
                    / 2.)
                    * 8.
            // The sprites go past the scale of the object, which is just 1x1 with an 8x8 sprite
            {
                // We only check for attachments using the children, since we created a seperate child
                // on top of the root in the beginning
                commands.entity(root_entity).add_child(attachable_entity);
                commands.entity(attachable_entity).insert(Player);
                commands.entity(attachable_entity).remove::<Object>();

                // The new translations are offsets from the parent
                attachable_transform.translation = Vec3::new(
                    attachable_transform.translation.x - root_transform.translation.x,
                    attachable_transform.translation.y - root_transform.translation.y,
                    0.,
                );

                root_stats.size += 1;
                root_stats.health += 10;
            }
        }
    }
}

pub fn check_player_death_system(
    mut commands: Commands,
    mut query: Query<(&Stats, Entity), With<PlayerRoot>>,
) {
    for (stats, entity) in query.iter_mut() {
        if stats.health <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
