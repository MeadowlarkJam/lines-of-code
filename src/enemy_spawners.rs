use crate::components::*;
use bevy::prelude::*;

pub fn spawn_shieldy(mut commands: Commands, windows: Res<Windows>) {
    let position = Vec3::new(0., 0., 999.);

    let root = commands
        .spawn()
        .insert(Collider {})
        .insert(Enemy {})
        .insert(EnemyRoot {})
        .insert(Stats {
            size: 7,
            health: 100,
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: position,
                scale: Vec3::new(20., 20., 20.),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Velocity {
            x: 0.,
            y: 0.,
            rotation: 0.4,
        })
        .id();
    // Arms and attach them to the root
    for i in 1..=2 {
        let element_right = commands
            .spawn()
            .insert(Collider {})
            .insert(Enemy {})
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(i as f32, 0., 0.),
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::from_rotation_z(
                        rand::random::<f32>() * 2. * std::f32::consts::PI,
                    ),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0., 1., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        let element_left = commands
            .spawn()
            .insert(Collider {})
            .insert(Enemy {})
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-i as f32, 0., 0.),
                    scale: Vec3::new(1., 1., 1.),
                    rotation: Quat::from_rotation_z(
                        rand::random::<f32>() * 2. * std::f32::consts::PI,
                    ),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0., 1., 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .id();

        commands.entity(root).add_child(element_right);
        commands.entity(root).add_child(element_left);
    }
    let shield_right = commands
        .spawn()
        .insert(Collider {})
        .insert(Enemy {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(2., 1., 0.),
                scale: Vec3::new(1., 1., 1.),
                rotation: Quat::from_rotation_z(rand::random::<f32>() * 2. * std::f32::consts::PI),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    let shield_left = commands
        .spawn()
        .insert(Collider {})
        .insert(Enemy {})
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-2., -1., 0.),
                scale: Vec3::new(1., 1., 1.),
                rotation: Quat::from_rotation_z(rand::random::<f32>() * 2. * std::f32::consts::PI),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0., 1., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands.entity(root).add_child(shield_right);
    commands.entity(root).add_child(shield_left);
}
