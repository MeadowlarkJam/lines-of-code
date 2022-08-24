use std::f32::consts::TAU;

use bevy::{
    diagnostic::EntityCountDiagnosticsPlugin, prelude::*, render::texture::ImageSettings,
    time::FixedTimestep,
};
use bevy_editor_pls::prelude::*;

const PLAYER_SPEED: f32 = 2.;

mod components;
use components::*;
mod attachment_systems;
use attachment_systems::*;
mod enemy_spawners;
use enemy_spawners::*;
mod nodes;
use nodes::*;
mod events;
use events::*;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_event::<Hit>()
        .add_event::<SoundEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_startup_system(load_assets.before(setup))
        .add_startup_system(setup.before(spawn_area))
        .add_startup_system(spawn_area)
        .add_startup_system(spawn_shieldy)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(move_player.before(check_attachment))
                .with_system(move_objects.before(check_attachment))
                .with_system(velocity_dropoff.after(move_objects))
                .with_system(check_attachment),
        )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1.))
        //         .with_system(spawn_object),
        // )
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(1. / 2.))
        //         .with_system(clean_objects)
        //         .with_system(spawn_shieldy),
        // )
        .add_system(camera_follow.after(move_player))
        .add_system(shoot_player_zapper)
        .add_system(check_hits.after(shoot_player_zapper))
        .add_system(check_death.after(check_hits))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 4.))
                .with_system(remove_zap_effect),
        )
        .run();
}

fn load_assets(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    asset_server.load::<Image, &str>("zapper.png");
    asset_server.load::<Image, &str>("debris.png");
    asset_server.load::<Image, &str>("player.png");
    asset_server.load::<Image, &str>("forcefield.png");
    asset_server.load::<Image, &str>("shield.png");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Init the camera entity
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                scale: Vec3::new(0.25, 0.25, 0.25),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

pub fn spawn_area(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_handle = asset_server.get_handle("player.png");
    // Create a player that is on top of the root. This makes sure that we only need to attach to other non-root blocks and can query for the root-transform later on
    let player_root_entity = commands
        .spawn()
        .insert(Collider {})
        .insert(Player {})
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
        .insert(Collider {})
        .insert(Player {})
        .insert(PlayerRoot {})
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

    // Place two zappers
    let zapper_handle = asset_server.get_handle("zapper.png");

    let starter_zapper = spawn_laser_turret(
        &mut commands,
        Vec3::new(
            rand::random::<f32>() * 200. - 100.,
            rand::random::<f32>() * 200. - 100.,
            0.,
        ),
        0.,
        zapper_handle.clone(),
        Zapper {
            damage: 10,
            fire_rate: 1.,
            cooldown_timer: 0.,
            range: 100.,
        },
    );

    commands.entity(starter_zapper).insert(Object {});

    let starter_zapper = spawn_laser_turret(
        &mut commands,
        Vec3::new(
            rand::random::<f32>() * 200. - 100.,
            rand::random::<f32>() * 200. - 100.,
            0.,
        ),
        0.,
        zapper_handle.clone(),
        Zapper {
            damage: 10,
            fire_rate: 1.,
            cooldown_timer: 0.,
            range: 100.,
        },
    );

    commands.entity(starter_zapper).insert(Object {});

    // Some uniformly distributed debris around the player
    let debris_handle: Handle<Image> = asset_server.get_handle("debris.png");

    for _ in 0..5 {
        // Spawn a node with debris
        let debris = spawn_empty_node(
            &mut commands,
            Vec3::new(
                rand::random::<f32>() * 200. - 100.,
                rand::random::<f32>() * 200. - 100.,
                0.,
            ),
            rand::random::<f32>() * TAU,
            debris_handle.clone(),
        );

        commands.entity(debris).insert(Object {}).insert(Velocity {
            x: 0.,
            y: 0.,
            rotation: rand::random::<f32>() * 0.2,
        });
    }
}

// Systems can query data in an SQL-like fashion
//
// Update the player position
fn move_player(
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
    player_transform.translation.x = (player_transform.translation.x + movement_x * PLAYER_SPEED);
    player_transform.translation.y = (player_transform.translation.y + movement_y * PLAYER_SPEED);
}

// Currently the camera_query returns several cameras and crashes
fn camera_follow(
    player_query: Query<&Transform, (With<PlayerRoot>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerRoot>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}

fn move_objects(
    mut query: Query<
        (&mut Transform, &mut Velocity),
        (With<Object>, Without<Parent>, Without<Player>),
    >,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += 1. * velocity.x as f32;
        transform.translation.y += 1. * velocity.y as f32;
        let (_, _, z) = transform.rotation.to_euler(EulerRot::XYZ);
        transform.rotation = Quat::from_rotation_z(z + velocity.rotation);
    }
}

fn spawn_object(
    mut commands: Commands,
    windows: Res<Windows>,
    player_query: Query<&Transform, With<PlayerRoot>>,
) {
    let player_transform = player_query.single();
    let width = windows.primary().width() as f32;
    let height = windows.primary().height() as f32;

    // Random position on the screen
    let x = player_transform.translation.x + rand::random::<f32>() * width - width / 2.;
    let y = player_transform.translation.y + rand::random::<f32>() * height - height / 2.;
    let position = Vec3::new(x, y, 0.);

    commands
        .spawn()
        .insert(Collider {})
        .insert(Object {})
        .insert(Enemy {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: position,
                scale: Vec3::new(20.0, 20.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1., 0.2, 0.),
                ..default()
            },
            ..default()
        })
        .insert(Velocity {
            x: rand::random::<f32>() * 2. - 1.,
            y: rand::random::<f32>() * 2. - 1.,
            rotation: rand::random::<f32>() * 0.2 - 0.1,
        })
        .insert(Stats { size: 1, health: 1 });
}

fn shoot_player_zapper(
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
                            .insert(ZapEffect {});
                    }

                    // Only one shot per cooldown
                    break;
                }
            }
        }
    }
}

fn remove_zap_effect(mut commands: Commands, mut query: Query<Entity, With<ZapEffect>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn check_hits(
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

fn check_death(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Stats, Entity), (With<PlayerRoot>, Without<EnemyRoot>)>,
    mut enemy_query: Query<(&mut Stats, Entity, &EnemyRoot, &Transform), Without<PlayerRoot>>,
) {
    for (mut enemy_stats, enemy_entity, enemy_root, enemy_transform) in enemy_query.iter_mut() {
        if enemy_stats.health <= 0 {
            let debris_handle = asset_server.get_handle("debris.png");
            match enemy_root.enemy_type {
                EnemyType::Shieldy => {
                    let shield_handle = asset_server.get_handle("shield.png");
                    let forcefield_handle = asset_server.get_handle("forcefield.png");
                    // Drop a shield and 2 debris
                    let shield = spawn_shield_node(
                        &mut commands,
                        enemy_transform.translation,
                        0.,
                        shield_handle,
                        forcefield_handle,
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
                            enemy_transform.translation,
                            rand::random::<f32>() * TAU,
                            debris_handle.clone(),
                        );

                        commands.entity(debris).insert(Object {}).insert(Velocity {
                            x: rand::random::<f32>() * 2. - 1.,
                            y: rand::random::<f32>() * 2. - 1.,
                            rotation: rand::random::<f32>() * 0.2,
                        });
                    }
                }
                EnemyType::Zappy => todo!(),
                EnemyType::Boomy => todo!(),
            }
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}

fn velocity_dropoff(mut query: Query<(&mut Velocity), With<Object>>) {
    for (mut velocity) in query.iter_mut() {
        velocity.x *= 0.99;
        velocity.y *= 0.99;
        velocity.rotation *= 0.99;
    }
}

// Clean all the objects that are the length of the diagonal of the screen away from the player
fn clean_objects(
    mut commands: Commands,
    windows: Res<Windows>,
    object_query: Query<(Entity, &Transform), (With<Object>, Without<Children>, Without<Parent>)>,
    player_query: Query<&Transform, With<PlayerRoot>>,
) {
    let distance = (windows.primary().width().powf(2.) as f32
        + windows.primary().height().powf(2.) as f32)
        .sqrt();

    let player_transform = player_query.single();

    for entity in object_query.iter() {
        if player_transform.translation.distance(entity.1.translation) > distance * 2. {
            commands.entity(entity.0).despawn_recursive();
        }
    }
}
