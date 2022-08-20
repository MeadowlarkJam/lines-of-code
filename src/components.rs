use bevy::prelude::*;

// An empty component is basically just a flag for the querying
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Object {}

#[derive(Component)]
pub struct Item {}

#[derive(Component)]
pub struct Collider {}

#[derive(Component)]
pub struct Attached {}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
}

// Components can hold data too, though
#[derive(Component)]
pub struct Stats {
    pub size: f32,
}
