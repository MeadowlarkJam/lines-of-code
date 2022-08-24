use crate::{
    components::{Collider, Object, Stats, Velocity, Zapper},
    consts::{ASSET_SPRITES_DEBRIS, ASSET_SPRITES_ZAPPER},
    enemy::Enemy,
    nodes::{spawn_empty_node, spawn_laser_turret},
    player::{Player, PlayerRoot},
};
use bevy::prelude::*;
use std::f32::consts::TAU;

pub fn move_objects_system(
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

pub fn _spawn_object_system(
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
        .insert(Collider)
        .insert(Object)
        .insert(Enemy)
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
pub fn _clean_objects_system(
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

pub fn spawn_start_objects_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Place two zappers
    let zapper_handle = asset_server.get_handle(ASSET_SPRITES_ZAPPER);

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

    commands.entity(starter_zapper).insert(Object);

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

    commands.entity(starter_zapper).insert(Object);

    // Some uniformly distributed debris around the player
    let debris_handle: Handle<Image> = asset_server.get_handle(ASSET_SPRITES_DEBRIS);

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

        commands.entity(debris).insert(Object).insert(Velocity {
            x: 0.,
            y: 0.,
            rotation: rand::random::<f32>() * 0.2,
        });
    }
}
