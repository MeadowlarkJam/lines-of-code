use bevy::prelude::*;

pub struct Hit {
    pub damage: u32,
    pub target: Entity,
}
