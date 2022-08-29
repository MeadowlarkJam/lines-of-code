use super::CustomMaterial;
use bevy::prelude::*;

#[derive(Component)]
pub struct Starfield {
    pub handle: Handle<CustomMaterial>,
}
