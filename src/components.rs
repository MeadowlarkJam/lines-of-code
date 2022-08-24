use bevy::prelude::*;

// An empty component is basically just a flag for the querying
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerRoot {}

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct EnemyRoot {}

#[derive(Component)]
pub struct Object {}

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct Collider {}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

// Components can hold data too, though
// size: Amount of connected blocks
#[derive(Component)]
pub struct Stats {
    pub size: i32,
    pub health: i32,
}

#[derive(Component)]
pub struct MainCamera;

// Powerup nodes
#[derive(Component)]
pub struct Shield {
    // Health of the shield until the cooldown starts
    pub health: i32,
    // Cooldown in seconds
    pub cooldown: f32,
    pub cooldown_timer: f32,
}

#[derive(Component)]
pub struct ShieldForcefield {
    pub active: bool,
}

#[derive(Component)]
pub struct Laser {
    pub damage: i32,
    pub fire_rate: f32,
    pub cooldown_timer: f32,
}
