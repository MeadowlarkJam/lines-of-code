use bevy::prelude::*;

#[derive(Component)]
pub struct Object;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct Collider;

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
pub struct Zapper {
    pub damage: i32,
    pub fire_rate: f32,
    pub cooldown_timer: f32,
    pub range: f32,
}

#[derive(Component)]
pub struct ZapEffect;

#[derive(Component)]
pub struct Bullet {
    pub damage: i32,
    pub enemy: bool,
}

#[derive(Component)]
pub struct Cannon {
    pub damage: i32,
    pub fire_rate: f32,
    pub cooldown_timer: f32,
    pub range: f32,
}

#[derive(Component)]
pub struct Projectile {}

pub struct WorldStats {
    pub kills: i32,
    pub enemies_alive: i32
}