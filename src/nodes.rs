use crate::components::*;
use bevy::prelude::*;

pub fn spawn_empty_node<'a>(
    commands: &'a mut Commands,
    position: Vec3,
    rotation: f32,
    asset: Handle<Image>,
) -> Entity {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: Quat::from_rotation_z(rotation),
                ..default()
            },
            texture: asset,
            ..default()
        })
        .id()
}

pub fn spawn_shield_node<'a>(
    commands: &'a mut Commands,
    position: Vec3,
    rotation: f32,
    asset: Handle<Image>,
    shield_stats: Shield,
) -> Entity {
    // Spawn the forcefield and add it as a child to the shield
    let forcefield = spawn_shield_forcefield(
        commands,
        Vec3::new(0., 0., 0.),
        std::f32::consts::PI / 4.,
        Vec3::new(30., 30., 1.),
    );
    let shield_node = spawn_empty_node(commands, position, rotation, asset);
    commands
        .entity(shield_node)
        .insert(shield_stats)
        .add_child(forcefield)
        .id()
}

pub fn spawn_shield_forcefield<'a>(
    commands: &'a mut Commands,
    position: Vec3,
    rotation: f32,
    scale: Vec3,
) -> Entity {
    let forcefield_node = commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: position,
                scale,
                rotation: Quat::from_rotation_z(rotation),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgba(0., 0., 1., 0.5),
                ..default()
            },
            ..default()
        })
        .id();

    commands
        .entity(forcefield_node)
        .insert(ShieldForcefield { active: true })
        .id()
}

pub fn spawn_laser_turret<'a>(
    commands: &'a mut Commands,
    position: Vec3,
    rotation: f32,
    asset: Handle<Image>,
    stats: Zapper,
) -> Entity {
    let turret = spawn_empty_node(commands, position, rotation, asset);

    commands.entity(turret).insert(stats).id()
}
