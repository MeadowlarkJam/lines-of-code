use bevy::prelude::*;

// An empty component is basically just a flag for the querying
#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct Bullet {}

#[derive(Component)]
pub struct Item {}

// Components can hold data too, though
#[derive(Component)]
pub struct Stats {
    pub health: i32,
}
