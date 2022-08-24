use crate::{camera::MainCamera, player::PlayerRoot};
use bevy::prelude::*;

pub fn spawn_camera_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<PlayerRoot>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerRoot>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
