use super::{Enemy, EnemyRoot, EnemyType};
use crate::{
    asset::SpriteHandles,
    components::*,
    nodes::{spawn_cannon_node, spawn_empty_node, spawn_shield_node, spawn_zapper_node},
};
use bevy::prelude::*;

pub fn spawn_shieldy(mut commands: Commands, sprite_handles: Res<SpriteHandles>, position: Vec3) {
    let root = spawn_empty_node(&mut commands, position, 0., sprite_handles.debris.clone());
    commands
        .entity(root)
        .insert(Collider)
        .insert(Enemy)
        .insert(EnemyRoot {
            enemy_type: EnemyType::Shieldy,
        })
        .insert(Properties {
            size: 7,
            health: 70,
        });

    // Arms and attach them to the root
    for i in 1..=2 {
        let element_right = spawn_empty_node(
            &mut commands,
            Vec3::new(i as f32 * 8., 0., 0.),
            rand::random::<f32>() * 2. * std::f32::consts::PI,
            sprite_handles.debris.clone(),
        );
        commands
            .entity(element_right)
            .insert(Collider)
            .insert(Enemy);

        let element_left = spawn_empty_node(
            &mut commands,
            Vec3::new(-i as f32 * 8., 0., 0.),
            rand::random::<f32>() * 2. * std::f32::consts::PI,
            sprite_handles.debris.clone(),
        );
        commands.entity(element_left).insert(Collider).insert(Enemy);

        commands.entity(root).add_child(element_right);
        commands.entity(root).add_child(element_left);
    }
    let shield_right = spawn_shield_node(
        &mut commands,
        Vec3::new(16., 8., 0.),
        rand::random::<f32>() * 2. * std::f32::consts::PI,
        sprite_handles.shield.clone(),
        sprite_handles.forcefield.clone(),
        ShieldForcefield {
            health: 10,
            cooldown: 3.,
            cooldown_timer: 0.,
        },
        true,
    );

    commands.entity(shield_right).insert(Collider).insert(Enemy);

    let shield_left = spawn_shield_node(
        &mut commands,
        Vec3::new(-16., -8., 0.),
        rand::random::<f32>() * 2. * std::f32::consts::PI,
        sprite_handles.shield.clone(),
        sprite_handles.forcefield.clone(),
        ShieldForcefield {
            health: 10,
            cooldown: 3.,
            cooldown_timer: 0.,
        },
        true,
    );
    commands.entity(shield_left).insert(Collider).insert(Enemy);

    commands.entity(root).add_child(shield_right);
    commands.entity(root).add_child(shield_left);
}

pub fn spawn_zappy(mut commands: Commands, sprite_handles: Res<SpriteHandles>, position: Vec3) {
    let root = spawn_empty_node(&mut commands, position, 0., sprite_handles.debris.clone());

    commands
        .entity(root)
        .insert(Collider)
        .insert(Enemy)
        .insert(EnemyRoot {
            enemy_type: EnemyType::Zappy,
        })
        .insert(Properties {
            size: 9,
            health: 90,
        });
    for i in -1..=1 {
        for j in -1..=1 {
            let element;
            if (j == 1 || j == -1) && i == 0 {
                element = spawn_zapper_node(
                    &mut commands,
                    Vec3::new(0., j as f32 * 8., 0.),
                    0.,
                    sprite_handles.zapper.clone(),
                    Zapper {
                        damage: 10,
                        fire_rate: 1.,
                        cooldown_timer: 0.,
                        range: 100.,
                    },
                );

                commands.entity(element).insert(Collider).insert(Enemy);
            } else {
                element = spawn_empty_node(
                    &mut commands,
                    Vec3::new(i as f32 * 8., j as f32 * 8., 0.),
                    rand::random::<f32>() * 2. * std::f32::consts::PI,
                    sprite_handles.debris.clone(),
                );

                commands.entity(element).insert(Collider).insert(Enemy);
            }

            commands.entity(root).add_child(element);
        }
    }
}

pub fn spawn_boomy(mut commands: Commands, sprite_handles: Res<SpriteHandles>, position: Vec3) {
    let root = spawn_empty_node(&mut commands, position, 0., sprite_handles.debris.clone());

    commands
        .entity(root)
        .insert(Collider)
        .insert(Enemy)
        .insert(EnemyRoot {
            enemy_type: EnemyType::Boomy,
        })
        .insert(Properties {
            size: 9,
            health: 90,
        });
    for i in -1..=1 {
        for j in -1..=1 {
            let element;
            if j == 0 && i == 0 {
                element = spawn_cannon_node(
                    &mut commands,
                    Vec3::new(0., 0., 0.),
                    0.,
                    sprite_handles.cannon.clone(),
                    Cannon {
                        damage: 10,
                        fire_rate: 1.,
                        cooldown_timer: 0.,
                        range: 100.,
                    },
                );

                commands.entity(element).insert(Collider).insert(Enemy);
            } else {
                element = spawn_empty_node(
                    &mut commands,
                    Vec3::new(i as f32 * 8., j as f32 * 8., 0.),
                    rand::random::<f32>() * 2. * std::f32::consts::PI,
                    sprite_handles.debris.clone(),
                );

                commands.entity(element).insert(Collider).insert(Enemy);
            }

            commands.entity(root).add_child(element);
        }
    }
}
