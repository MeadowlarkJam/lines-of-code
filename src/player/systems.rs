use super::{Player, PlayerRoot, PlayerSizeIncreased};
use crate::{
    camera::MainCamera,
    components::{
        Bullet, Cannon, Collider, Object, Projectile, Properties, Shield, Velocity, ZapEffect,
        Zapper,
    },
    consts::{ASSET_SPRITES_PLAYER, PLAYER_SPEED},
    enemy::{Enemy, EnemyRoot},
    events::Hit,
    schedule::GameState,
    starfield::{CustomMaterial, Starfield},
};
use bevy::{prelude::*, render::camera::RenderTarget};

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
        .insert(Properties {
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

pub fn rotate_player_system(
    windows: Res<Windows>,
    mut player_query: Query<(&mut Transform, &Properties), With<PlayerRoot>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if let Some(cursor_position) = windows.primary().cursor_position() {
        let (camera, camera_transform) = camera_query.single();
        let (mut player_transform, player_stats) = player_query.single_mut();

        let window = if let RenderTarget::Window(id) = camera.target {
            windows.get(id).unwrap()
        } else {
            windows.get_primary().unwrap()
        };

        let window_size = Vec2::new(window.width() as f32, window.height() as f32);
        let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        let difference = world_pos - player_transform.translation;
        let angle = difference.y.atan2(difference.x);
        let new_rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);

        player_transform.rotation = player_transform
            .rotation
            .lerp(new_rotation, 0.1 / (player_stats.size as f32 * 0.1));
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
    mut event_hit: EventWriter<Hit>,
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
                    event_hit.send(Hit {
                        target: shootable_parent.get(),
                        damage: cannon_stats.damage,
                    });

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
    mut player_query: Query<(&mut Properties, Entity), (With<PlayerRoot>, Without<EnemyRoot>)>,
    mut enemy_query: Query<(&mut Properties, Entity), (With<EnemyRoot>, Without<PlayerRoot>)>,
) {
    let (mut player_stats, player_entity) = player_query.single_mut();
    for hit in event_hit.iter() {
        if hit.target == player_entity {
            player_stats.health = player_stats.health.saturating_sub(hit.damage);
        } else {
            match enemy_query.get_mut(hit.target) {
                Ok((mut enemy_properties, _)) => {
                    enemy_properties.health = enemy_properties.health.saturating_sub(hit.damage);
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
    mut query: Query<&Properties, With<PlayerRoot>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for properties in query.iter_mut() {
        if properties.health <= 0 {
            game_state.set(GameState::EndScreen).unwrap();
        }
    }
}

//#########################
//ATTACHMENT
//#########################

pub fn check_attachment_system(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, (With<Player>, Without<PlayerRoot>)>,
    mut player_root_query: Query<(Entity, &Transform), With<PlayerRoot>>,
    mut attachable_query: Query<
        (Entity, &mut Transform),
        (With<Object>, Without<Player>, Without<PlayerRoot>),
    >,
    mut event_writer: EventWriter<PlayerSizeIncreased>,
) {
    // We need the transform of the root, since everything is relative to it and when adding children we need to revert it first
    let (root_entity, root_transform) = player_root_query.get_single_mut().unwrap();

    for player_global_transform in &player_query {
        for (attachable_entity, mut attachable_transform) in attachable_query.iter_mut() {
            let distance = player_global_transform
                .compute_transform()
                .translation
                .distance(attachable_transform.translation);

            // TODO: More exact collision detection
            if distance
                < ((player_global_transform.compute_transform().scale.x
                    + attachable_transform.scale.x)
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
                let x = attachable_transform.translation.x - root_transform.translation.x;
                let y = attachable_transform.translation.y - root_transform.translation.y;

                attachable_transform.translation = Vec3::new(x, y, 0.0);

                // FIXME:
                // This doesn't work currently. The object is not moved correctly when
                // looking up or down while collecting it.
                attachable_transform.rotate_around(Vec3::ZERO, root_transform.rotation);

                event_writer.send(PlayerSizeIncreased);
            }
        }
    }
}

pub fn update_player_properties_system(
    mut query: Query<&mut Properties, With<PlayerRoot>>,
    event_reader: EventReader<PlayerSizeIncreased>,
) {
    let mut properties = query.single_mut();
    let player_size_increase = event_reader.len() as u32;
    event_reader.clear();

    if player_size_increase > 0 {
        properties.size += player_size_increase;
        properties.health += player_size_increase * 10;
    }
}
