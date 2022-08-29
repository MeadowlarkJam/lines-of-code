use super::{
    camera_follow_system,
    resources::RandomNumberResource,
    systems::{camera_zoom_system, spawn_camera_system},
};
use crate::{player::PlayerSystem, schedule::GameState};
use bevy::prelude::*;
use rand::Rng;

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
                .with_system(camera_follow_system)
                .with_system(camera_zoom_system),
        )
        .insert_resource(RandomNumberResource {
            rand1: {
                let r1 = rand::thread_rng().gen_range(500.0f32..1000.0f32);
                let r2 = (rand::thread_rng().gen_range(0.0f32..1.0f32).round() * 2.0) - 1.0;
                r1 * r2
            },

            rand2: {
                let r1 = rand::thread_rng().gen_range(500.0f32..1000.0f32);
                let r2 = (rand::thread_rng().gen_range(0.0f32..1.0f32).round() * 2.0) - 1.0;
                r1 * r2
            },
        });
    }
}
