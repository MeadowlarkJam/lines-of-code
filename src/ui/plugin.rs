use super::{
    components::{OnMainMenuScreen, OnSplashScreen},
    systems::{
        spawn_main_menu_system, spawn_splash_screen_system, update_main_menu_system,
        update_splash_screen_system,
    },
};
use crate::{despawn_recursive::despawn_entities_recursive_system, schedule::GameState};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct UiSystem;

/// The plugin that handles the user interface.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            // Splash Screen
            .add_system_set(
                SystemSet::on_enter(GameState::SplashScreen)
                    .label(UiSystem)
                    .with_system(spawn_splash_screen_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::SplashScreen)
                    .label(UiSystem)
                    .with_system(update_splash_screen_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::SplashScreen)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnSplashScreen>),
            )
            // Main Menu
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(spawn_main_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(update_main_menu_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnMainMenuScreen>),
            );
    }
}
