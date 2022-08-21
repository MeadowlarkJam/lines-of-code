use bevy::{prelude::*, time::FixedTimestep};

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 1.0);
const PLAYER_SIZE: Vec3 = Vec3::new(20.0, 20.0, 0.0);
const PLAYER_SPEED: f32 = 3.;

mod components;
use components::*;
mod absorption_systems;
use absorption_systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
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
        .run();
}

fn setup(mut commands: Commands) {
    // Init the camera entity
    commands.spawn_bundle(Camera2dBundle::default());

    // Create a player that is on top of the root. This makes sure that we only need to attach to other non-root blocks and can query for the root-transform later on
    let player_root_entity = commands
        .spawn()
        .insert(Collider {})
        .insert(Player {})
        .insert(Stats { size: 1 })
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
        .insert(Stats { size: 1 })
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
        })
        .add_child(player_root_entity);
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
    player_transform.translation.x =
        (player_transform.translation.x + movement_x * PLAYER_SPEED).clamp(left_bound, right_bound);
    player_transform.translation.y =
        (player_transform.translation.y + movement_y * PLAYER_SPEED).clamp(bottom_bound, top_bound);
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
        .insert(Stats { size: 1 });
}

// fn check_collisions(
//     mut commands: Commands,
//     collider_query: Query<
//         (
//             Entity,
//             &Transform,
//             Option<&Parent>,
//             &mut Stats,
//             Option<&Player>,
//         ),
//         (With<Collider>,),
//     >,
// ) {
//     for (entity, transform, parent, stats, is_player) in collider_query.iter() {
//         for (other_entity, other_transform, other_parent, other_stats, other_is_player) in
//             collider_query.iter()
//         {
//             if entity == other_entity {
//                 continue;
//             }

//             // Check if the entities are attached to the same parent
//             if let (Some(parent), Some(other_parent)) = (parent, other_parent) {
//                 if parent.id() == other_parent.id() {
//                     continue;
//                 }
//             }

//             // Check if the entities are overlapping
//             let distance = transform.translation.distance(other_transform.translation);
//             // TODO: Proper distance calculation
//             if distance < 20. {
//                 // Deciding who wins:
//                 // The clump with the bigger size value wins
//                 // During ties the player wins, other wise it's just the first entity
//                 //
//                 // What to do when something wins
//                 // - The winner gets the size of the loser added to it
//                 // - Iterate over every object of the loser, set the parent to the winner
//                 // - (In the case of the player, add the Player component to every object of the loser)

//                 // Check if the player is involved
//                 if is_player.is_some() || other_is_player.is_some() {
//                     // In a tie the player always wins
//                     // Check which one is the player
//                     match is_player.is_some() {
//                         true => {
//                             if stats.size >= other_stats.size {
//                                 // The player wins
//                                 // Add the size of the loser to the player
//                                 commands.entity(entity).insert(Stats {
//                                     size: stats.size + other_stats.size,
//                                 });
//                             }
//                         }
//                         false => {}
//                     }
//                 }
//             }
//         }
//     }
// }
