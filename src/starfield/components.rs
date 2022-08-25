use bevy::prelude::*;

use super::CustomMaterial;

#[derive(Component)]
pub struct Starfield {
    pub handle: Handle<CustomMaterial>,
}
