use super::{
    components::{OnIngameScreen, OnMainMenuScreen, OnPausedScreen, OnSplashScreen},
    systems::{
        button_highlight_system, check_for_paused_system, check_for_unpaused_system,
        main_menu_button_interaction_system, paused_button_interaction_system,
        spawn_ingame_ui_system, spawn_main_menu_ui_system, spawn_paused_ui_system,
        spawn_splash_screen_system, update_splash_screen_system, update_ui_health_system,
        update_ui_score_system,
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
        app.add_system(button_highlight_system)
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
                    .with_system(spawn_main_menu_ui_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(main_menu_button_interaction_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnMainMenuScreen>),
            )
            // InGame
            .add_system_set(
                SystemSet::on_enter(GameState::InGame)
                    .label(UiSystem)
                    .with_system(spawn_ingame_ui_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(UiSystem)
                    .with_system(update_ui_score_system)
                    .with_system(update_ui_health_system)
                    .with_system(check_for_paused_system),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::MainMenu)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnIngameScreen>),
            )
            // Paused
            .add_system_set(
                SystemSet::on_enter(GameState::Paused)
                    .label(UiSystem)
                    .with_system(spawn_paused_ui_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .label(UiSystem)
                    .with_system(check_for_unpaused_system)
                    .with_system(paused_button_interaction_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Paused)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnPausedScreen>),
            );
    }
}
