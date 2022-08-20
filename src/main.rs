use bevy::{prelude::*, time::FixedTimestep};

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 1.0);
const PLAYER_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const PLAYER_SPEED: f32 = 3.;

mod components;
use components::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0 / 60.0))
            .with_system(move_player.before(check_collisions))
            .with_system(move_objects.before(check_collisions))
            .with_system(check_collisions)
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(spawn_object)
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Init the camera entity
    commands.spawn_bundle(Camera2dBundle::default());

    // Init the player entity
    commands
        .spawn()
        .insert(Collider {})
        .insert(Player{})
        .insert(Stats{size: 20.})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        });
}

// Systems can query data in an SQL-like fashion
//
// Update the player position
fn move_player(keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<Player>>) {
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
        let left_bound = - windows.primary().width() as f32 / 2. + player_transform.scale.x / 2.;
        let right_bound = windows.primary().width() as f32 / 2. - player_transform.scale.x / 2.;
        let bottom_bound = - windows.primary().height() as f32 / 2. + player_transform.scale.y / 2.;
        let top_bound = windows.primary().height() as f32 / 2. - player_transform.scale.y / 2.;

        // Move the player and clamp it to the screen
        player_transform.translation.x = (player_transform.translation.x + movement_x * PLAYER_SPEED).clamp(left_bound, right_bound);
        player_transform.translation.y = (player_transform.translation.y + movement_y * PLAYER_SPEED).clamp(bottom_bound, top_bound);
    }

fn move_objects(mut query: Query<(&mut Transform, &mut Velocity), With<Object>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += 1. * velocity.x as f32;
        transform.translation.y += 1. * velocity.y as f32;
        let (_,_,z) = transform.rotation.to_euler(EulerRot::XYZ);
        transform.rotation = Quat::from_rotation_z(z + velocity.rotation);
    }
}

fn spawn_object(mut commands: Commands,
    windows: Res<Windows>,) {
    let width = windows.primary().width() as f32;
    let height = windows.primary().height() as f32;

    // Randome position on the screen
    let x = rand::random::<f32>() * width - width / 2.;
    let y = rand::random::<f32>() * height - height / 2.;
    let position = Vec3::new(x, y, 0.);

    commands
        .spawn()
        .insert(Collider {})
        .insert(Object{})
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
        });
}

fn check_collisions(
    mut commands: Commands,
    mut player_query: Query<(&mut Stats, &mut Transform), With<Player>>,
    object_query: Query<(Entity, &Transform), (With<Object>, With<Collider>, Without<Player>)>,
) {
    // Get the player's position
    let (mut player_stats, mut player_transform) = player_query.single_mut();

    // Check for collisions with objects
    for (object_entity, object_transform) in object_query.iter() {
        // Check if the player and the object are colliding
        if player_transform.translation.distance(object_transform.translation) < ((player_transform.scale.x + object_transform.scale.x) as f32) / 2. {
            // Remove the object
            commands.entity(object_entity).despawn();

            // Increase the player's size
            player_stats.size += 1.;
            player_transform.scale = Vec3::new(player_stats.size as f32, player_stats.size as f32, 0.);
        }
    }
}