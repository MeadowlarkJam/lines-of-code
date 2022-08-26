use super::{Player, PlayerRoot};
use crate::{
    components::{
        Bullet, Cannon, Collider, Object, Projectile, Shield, Stats, Velocity, ZapEffect, Zapper,
    },
    consts::{ASSET_SPRITES_PLAYER, PLAYER_SPEED},
    enemy::{Enemy, EnemyRoot},
    events::Hit,
    nodes::spawn_empty_node,
    starfield::{CustomMaterial, Starfield}, schedule::GameState,
};
use bevy::prelude::*;

pub fn spawn_player_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle = asset_server.get_handle(ASSET_SPRITES_PLAYER);

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
    mut starfield: Query<(&mut Starfield, &mut Transform), Without<PlayerRoot>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // The player's movement directions
    let mut movement_x = 0.;
    let mut movement_y = 0.;

    let mut player_transform = query.single_mut();

    // Add the different directions. This way pressing left and right cancels out
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        movement_x -= 1.;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        movement_x += 1.;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        movement_y -= 1.;
    }
    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        movement_y += 1.;
    }

    // Move the player and clamp it to the screen
    player_transform.translation.x = player_transform.translation.x + movement_x * PLAYER_SPEED;
    player_transform.translation.y = player_transform.translation.y + movement_y * PLAYER_SPEED;
    let (sf, mut tf) = starfield.single_mut();
    tf.translation.x = player_transform.translation.x;
    tf.translation.y = player_transform.translation.y;
    if let Some(custom_material) = materials.get_mut(&sf.handle) {
        custom_material.pos = Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        );
    }
}

//#####################
// PLAYER TURRETS
//#####################
pub fn shoot_player_zapper_system(
    mut commands: Commands,
    time: Res<Time>,
    mut event_hit: EventWriter<Hit>,
    mut zapper_query: Query<(&GlobalTransform, &mut Zapper), With<Player>>,
    shootable_query: Query<(&GlobalTransform, Entity, &Parent), With<Enemy>>,
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

pub fn shoot_player_cannon_system(
    mut commands: Commands,
    time: Res<Time>,
    mut cannon_query: Query<(&GlobalTransform, &mut Cannon), With<Player>>,
    shootable_query: Query<(&GlobalTransform, Entity, &Parent), With<Enemy>>,
) {
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
                                color: Color::rgb(1., 1., 0.),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(Projectile {})
                        .insert(Bullet {
                            damage: cannon_stats.damage,
                            enemy: false,
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

pub fn player_bullet_collision(
    mut commands: Commands,
    mut event_hit: EventWriter<Hit>,
    enemy_query: Query<(Entity, &GlobalTransform), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut forcefield_query: Query<
        (&GlobalTransform, &mut Visibility, &mut Shield),
        (With<Enemy>, With<Parent>),
    >,
) {
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
    for (enemy_entity, enemy_transform) in enemy_query.iter() {
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

pub fn check_hits_system(
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

pub fn check_player_death_system(
    mut commands: Commands,
    mut query: Query<(&Stats, Entity), With<PlayerRoot>>,
    mut game_state: ResMut<State<GameState>>
) {
    for (stats, entity) in query.iter_mut() {
        if stats.health <= 0 {
            // TODO: Change visibility instead of despawning, which breaks the game
            // For now just quit to the MainMenu
            game_state.set(GameState::MainMenu).unwrap();
        }
    }
}

//#########################
//ATTACHMENT
//#########################

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
