use bevy::{diagnostic::EntityCountDiagnosticsPlugin, prelude::*, time::FixedTimestep, render::texture::ImageSettings};
use bevy_editor_pls::prelude::*;

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 1.0);
const PLAYER_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const PLAYER_SPEED: f32 = 3.;

mod components;
use components::*;
mod absorption_systems;
use absorption_systems::*;
mod enemy_spawners;
use enemy_spawners::*;
mod nodes;
use nodes::*;
mod asset_resources;
use asset_resources::*;

fn main() {
    App::new()
    .insert_resource(ImageSettings::default_nearest())

        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_startup_system(load_assets.before(setup))
        .add_startup_system(setup.before(spawn_shieldy))
        .add_startup_system(spawn_shieldy)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
                .with_system(move_player.before(check_attachment))
                .with_system(move_objects.before(check_attachment))
                .with_system(check_attachment),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(spawn_object),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1. / 2.))
                .with_system(clean_objects),
        )
        //.add_system(camera_follow.after(move_player))
        
        .run();
}

fn load_assets(mut commands: Commands,
               mut asset_server: Res<AssetServer>) {
    asset_server.load::<Image, &str>("zapper.png");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Init the camera entity
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);

    // Create a player that is on top of the root. This makes sure that we only need to attach to other non-root blocks and can query for the root-transform later on
    let player_root_entity = commands
        .spawn()
        .insert(Collider {})
        .insert(Player {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
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
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        })
        .add_child(player_root_entity);

    let zapper_handle = asset_server.get_handle("zapper.png");

    let starter_shield = spawn_shield_node(
        &mut commands,
        Vec3::new(100., 100., 0.),
        0.,
        Vec3::new(2., 2., 1.),
        zapper_handle.clone(),
        zapper_handle.clone(),
        Shield {
            health: 10,
            cooldown: 3.,
            cooldown_timer: 0.,
        },
    );

    commands.entity(starter_shield).insert(Object {});

    let starter_zapper = spawn_laser_turret(
        &mut commands,
        Vec3::new(-100., 100., 0.),
        0.,
        Vec3::new(2., 2., 1.),
        zapper_handle.clone(),
        Laser {
            damage: 10,
            fire_rate: 1.,
            cooldown_timer: 0.,
        },
    );

    commands.entity(starter_zapper).insert(Object {});
}

// Systems can query data in an SQL-like fashion
//
// Update the player position
fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
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

    // Get the bounds of the screen
    let left_bound = -windows.primary().width() as f32 / 2. + player_transform.scale.x / 2.;
    let right_bound = windows.primary().width() as f32 / 2. - player_transform.scale.x / 2.;
    let bottom_bound = -windows.primary().height() as f32 / 2. + player_transform.scale.y / 2.;
    let top_bound = windows.primary().height() as f32 / 2. - player_transform.scale.y / 2.;

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

fn spawn_object(mut commands: Commands, windows: Res<Windows>) {
    let width = windows.primary().width() as f32;
    let height = windows.primary().height() as f32;

    // Randome position on the screen
    let x = rand::random::<f32>() * width - width / 2.;
    let y = rand::random::<f32>() * height - height / 2.;
    let position = Vec3::new(x, y, 0.);

    commands
        .spawn()
        .insert(Collider {})
        .insert(Object {})
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
