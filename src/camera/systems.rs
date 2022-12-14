use crate::{
    camera::MainCamera,
    player::PlayerRoot,
    starfield::{CustomMaterial, Starfield},
};
use bevy::prelude::*;

use super::resources::RandomNumberResource;

pub fn spawn_camera_system(mut commands: Commands, random: Res<RandomNumberResource>) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(random.rand1, random.rand2, 1.0),
                scale: Vec3::new(0.25, 0.25, 0.25),
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<PlayerRoot>, Without<MainCamera>, Without<Starfield>)>,
    mut camera_query: Query<
        &mut Transform,
        (With<MainCamera>, Without<PlayerRoot>, Without<Starfield>),
    >,
    mut starfield: Query<
        (&mut Starfield, &mut Transform),
        (With<Starfield>, Without<MainCamera>, Without<PlayerRoot>),
    >,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    let lerped = camera_transform
        .translation
        .lerp(player_transform.translation, 0.1);

    camera_transform.translation.x = lerped.x;
    camera_transform.translation.y = lerped.y;

    // Move the starfield to the player position
    let (sf, mut tf) = starfield.single_mut();
    tf.translation.x = lerped.x;
    tf.translation.y = lerped.y;

    // let lerped = tf
    //     .translation
    //     .lerp(player_transform.translation, 0.1);

    if let Some(custom_material) = materials.get_mut(&sf.handle) {
        custom_material.pos = Vec4::new(lerped.x, lerped.y, 0.0, 0.0);
    }
}

pub fn camera_zoom_system(
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerRoot>)>,
    mut player_root_query: Query<&PlayerRoot, With<PlayerRoot>>,
) {
    let mut camera_transform = camera_query.single_mut();
    let root_component = player_root_query.get_single_mut().unwrap();

    let new_scale = 0.25 + (0.01 * (root_component.dist / 8.0));

    let lerped = camera_transform
        .scale
        .lerp(Vec3::new(new_scale, new_scale, 0.0), 0.1);

    camera_transform.scale.x = lerped.x;
    camera_transform.scale.y = lerped.y;
}
