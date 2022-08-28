use super::{
    components::{OnDeathScreen, OnIngameScreen, OnMainMenuScreen, OnPausedScreen, OnSplashScreen},
    systems::{
        button_highlight_system, end_screen_button_interaction_system, end_screen_death_sound,
        main_menu_button_interaction_system, paused_button_interaction_system,
        spawn_end_screen_ui_system, spawn_ingame_ui_system, spawn_main_menu_ui_system,
        spawn_paused_ui_system, spawn_splash_screen_system, update_splash_screen_system,
        update_ui_enemies_alive_system, update_ui_kills_system, update_ui_player_stats_system,
        update_ui_score_system, start_music, 
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
                    .with_system(spawn_splash_screen_system)
                    .with_system(start_music),
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
                    .with_system(update_ui_player_stats_system)
                    .with_system(update_ui_kills_system)
                    .with_system(update_ui_enemies_alive_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::InGame)
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
                    .with_system(paused_button_interaction_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Paused)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnPausedScreen>),
            )
            // End screen
            .add_system_set(
                SystemSet::on_enter(GameState::EndScreen)
                    .label(UiSystem)
                    .with_system(spawn_end_screen_ui_system)
                    .with_system(end_screen_death_sound),
            )
            .add_system_set(
                SystemSet::on_update(GameState::EndScreen)
                    .label(UiSystem)
                    .with_system(end_screen_button_interaction_system),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::EndScreen)
                    .label(UiSystem)
                    .with_system(despawn_entities_recursive_system::<OnDeathScreen>),
            );
    }
}
