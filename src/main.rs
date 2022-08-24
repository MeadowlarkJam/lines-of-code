use bevy::{diagnostic::EntityCountDiagnosticsPlugin, prelude::*, render::texture::ImageSettings};
use bevy_editor_pls::prelude::*;

mod components;
use components::*;
mod nodes;
use nodes::*;
mod events;
use events::*;

// Plugins
mod asset;
mod camera;
mod enemy;
mod object;
mod player;
mod schedule;

fn main() {
    App::new()
        // ----- Game -----
        .add_plugin(asset::AssetPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(object::ObjectPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(schedule::SchedulePlugin)
        // ----- Bevy -----
        .add_event::<Hit>()
        .add_event::<SoundEvent>()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityCountDiagnosticsPlugin)
        .add_system(bevy::window::close_on_esc)
        // ----- Third party -----
        .add_plugin(EditorPlugin)
        // ----- Start -----
        .run();
}
