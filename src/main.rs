use bevy::{
    diagnostic::EntityCountDiagnosticsPlugin, prelude::*, render::texture::ImageSettings,
    sprite::Material2dPlugin,
};
use bevy_editor_pls::prelude::*;

mod components;
mod consts;
mod despawn_recursive;
mod events;
mod nodes;
use consts::COLOR_BACKGROUND_DARK;
use events::*;
use starfield::CustomMaterial;
use stats::Stats;

// Plugins
mod asset;
mod camera;
mod enemy;
mod object;
mod player;
mod schedule;
mod starfield;
mod stats;
mod ui;

fn main() {
    App::new()
        // ----- Bevy -----
        .add_event::<Hit>()
        .add_event::<SoundEvent>()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(COLOR_BACKGROUND_DARK))
        .insert_resource(Stats::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityCountDiagnosticsPlugin)
        // ----- Game -----
        .add_plugin(asset::AssetPlugin)
        .add_plugin(Material2dPlugin::<CustomMaterial>::default())
        .add_plugin(camera::CameraPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(object::ObjectPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(schedule::SchedulePlugin)
        .add_plugin(starfield::StarfieldPlugin)
        .add_plugin(stats::StatsPlugin)
        .add_plugin(ui::UiPlugin)
        // ----- Third party -----
        .add_plugin(EditorPlugin)
        // ----- Start -----
        .run();
}
