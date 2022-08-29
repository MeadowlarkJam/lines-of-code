use bevy::{prelude::*, render::texture::ImageSettings, sprite::Material2dPlugin};

// Diagnostic imports
// use bevy::diagnostic::EntityCountDiagnosticsPlugin;
// use bevy_editor_pls::prelude::*;

mod components;
mod despawn_recursive;
mod events;
mod nodes;
use events::*;
use starfield::CustomMaterial;
mod colors;

// Plugins
mod asset;
mod audio;
mod camera;
mod enemy;
mod object;
mod player;
mod schedule;
mod starfield;
mod stats;
mod ui;
mod window;

#[allow(clippy::type_complexity)]
fn main() {
    App::new()
        // ----- Bevy -----
        .add_event::<Hit>()
        .insert_resource(ImageSettings::default_nearest())
        // ----- Plugins -----
        .add_plugin(window::WindowPlugin) // Has to be before the `DefaultPlugins`
        .add_plugins(DefaultPlugins)
        .add_plugin(asset::AssetPlugin)
        .add_plugin(audio::AudioPlugin)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(object::ObjectPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(schedule::SchedulePlugin)
        .add_plugin(starfield::StarfieldPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(ui::UiPlugin)
        // ----- Diagnostics -----
        // .add_plugin(EditorPlugin)
        // .add_plugin(EntityCountDiagnosticsPlugin)
        // ----- Start -----
        .run();
}
