use crate::{
    components::{
        Bullet, Cannon, Collider, Object, Projectile, Properties, Shield, Velocity, Zapper,
    },
    consts::{ASSET_SPRITES_CANNON, ASSET_SPRITES_DEBRIS, ASSET_SPRITES_ZAPPER},
    enemy::Enemy,
    events::Hit,
    nodes::{spawn_cannon_node, spawn_empty_node, spawn_zapper_node},
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
        .insert(Properties { size: 1, health: 1 });
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

    let starter_zapper = spawn_zapper_node(
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

    let starter_zapper = spawn_zapper_node(
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

    let starter_cannon = spawn_cannon_node(
        &mut commands,
        Vec3::new(
            rand::random::<f32>() * 200. - 100.,
            rand::random::<f32>() * 200. - 100.,
            0.,
        ),
        0.,
        asset_server.get_handle(ASSET_SPRITES_CANNON),
        Cannon {
            damage: 10,
            fire_rate: 1.,
            cooldown_timer: 0.,
            range: 100.,
        },
    );
    commands.entity(starter_cannon).insert(Object);

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

pub fn velocity_dropoff_system(mut query: Query<&mut Velocity, With<Object>>) {
    for mut velocity in query.iter_mut() {
        velocity.x *= 0.99;
        velocity.y *= 0.99;
        velocity.rotation *= 0.99;
    }
}

pub fn move_projectile(mut query: Query<(&mut Transform, &Velocity), With<Projectile>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += 1. * velocity.x as f32;
        transform.translation.y += 1. * velocity.y as f32;
    }
}

pub fn bullet_collision(
    mut commands: Commands,
    mut event_hit: EventWriter<Hit>,
    hittable_query: Query<
        (Entity, &GlobalTransform, Option<&Enemy>),
        Or<(With<Enemy>, With<Player>)>,
    >,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut forcefield_query: Query<
        (
            &GlobalTransform,
            &mut Visibility,
            &mut Shield,
            Option<&Enemy>,
        ),
        With<Parent>,
    >,
) {
    // Check for forcefield collision
    // On forcefield collision, just do the forcefield damage here and remove the bullet
    // Forcefield radius is 18 units
    for (forcefield_transform, mut forcefield_visibility, mut forcefield_stats, is_enemy) in
        forcefield_query.iter_mut()
    {
        for (bullet_entity, bullet_transform, bullet_stats) in bullet_query.iter() {
            let distance = forcefield_transform
                .compute_transform()
                .translation
                .distance(bullet_transform.translation);
            if distance < 18. && forcefield_visibility.is_visible {
                // If the bullet is by an enemy and the currently viewed forcefield is by an enemy, ignore it, as that would be a self hit
                if bullet_stats.enemy == is_enemy.is_some() {
                    continue;
                }
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
    for (hittable_entity, hittable_transform, is_enemy) in hittable_query.iter() {
        for (bullet_entity, bullet_transform, bullet_stats) in bullet_query.iter() {
            let distance = hittable_transform
                .compute_transform()
                .translation
                .distance(bullet_transform.translation);
            if distance < 5. {
                if bullet_stats.enemy == is_enemy.is_some() {
                    continue;
                }
                event_hit.send(Hit {
                    target: hittable_entity,
                    damage: bullet_stats.damage,
                });
                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }
}
