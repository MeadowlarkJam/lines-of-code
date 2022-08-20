use bevy::{prelude::*, time::FixedTimestep};

const PLAYER_COLOR: Color = Color::rgb(0.7, 0.7, 1.0);
const PLAYER_SIZE: Vec3 = Vec3::new(20.0, 40.0, 0.0);
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
            .with_system(move_player)
        )
        .run();
}

fn setup(mut commands: Commands) {
    // Init the camera entity
    commands.spawn_bundle(Camera2dBundle::default());

    // Init the player entity
    commands
        .spawn()
        .insert(Player{})
        .insert(Stats{health: 100})
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
        let left_bound = - windows.primary().width() as f32 / 2. + PLAYER_SIZE[0] / 2.;
        let right_bound = windows.primary().width() as f32 / 2. - PLAYER_SIZE[0] / 2.;
        let bottom_bound = - windows.primary().height() as f32 / 2. + PLAYER_SIZE[1] / 2.;
        let top_bound = windows.primary().height() as f32 / 2. - PLAYER_SIZE[1] / 2.;

        // Move the player and clamp it to the screen
        player_transform.translation.x = (player_transform.translation.x + movement_x * PLAYER_SPEED).clamp(left_bound, right_bound);
        player_transform.translation.y = (player_transform.translation.y + movement_y * PLAYER_SPEED).clamp(bottom_bound, top_bound);
    }