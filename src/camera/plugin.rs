use super::{camera_follow_system, systems::spawn_camera_system};
use crate::player::PlayerSystem;
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct CameraSystem;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera_system.label(CameraSystem))
            .add_system_set(
                SystemSet::new()
                    .label(CameraSystem)
                    .after(PlayerSystem)
                    .with_system(camera_follow_system),
            );
    }
}
