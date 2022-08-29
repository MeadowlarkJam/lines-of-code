use super::{camera_follow_system, systems::spawn_camera_system};
use crate::{player::PlayerSystem, schedule::GameState};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct CameraSystem;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::BeforeSplashScreen)
                .label(CameraSystem)
                .with_system(spawn_camera_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .label(CameraSystem)
                .after(PlayerSystem)
                .with_system(camera_follow_system),
        );
    }
}
