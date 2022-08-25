use bevy::{diagnostic::EntityCountDiagnosticsPlugin, prelude::*, render::texture::ImageSettings};
use bevy_editor_pls::prelude::*;

mod components;
mod consts;
mod despawn_recursive;
mod events;
mod nodes;
use consts::COLOR_BACKGROUND_DARKEST;
use events::*;

// Plugins
mod asset;
mod camera;
mod enemy;
mod object;
mod player;
mod schedule;
mod ui;

fn main() {
    App::new()
        // ----- Game -----
        .add_plugin(asset::AssetPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(object::ObjectPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(schedule::SchedulePlugin)
        .add_plugin(ui::UiPlugin)
        // ----- Bevy -----
        .add_event::<Hit>()
        .add_event::<SoundEvent>()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(COLOR_BACKGROUND_DARKEST))
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityCountDiagnosticsPlugin)
        // ----- Third party -----
        .add_plugin(EditorPlugin)
        // ----- Start -----
        .run();
}
