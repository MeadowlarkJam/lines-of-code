use super::{
    systems::{
        check_for_paused_system, check_for_unpaused_system, fade_in_system, fade_out_system,
        on_enter_after_end_screen_system, on_enter_after_in_game_system,
        on_enter_after_main_menu_system, on_enter_after_splash_screen_system,
        on_enter_before_end_screen_system, on_enter_before_in_game_system,
        on_enter_before_main_menu_system, on_enter_before_splash_screen_system,
        on_exit_after_end_screen_system, on_exit_after_in_game_system,
        on_exit_after_main_menu_system, on_exit_after_splash_screen_system,
        on_exit_before_end_screen_system, on_exit_before_in_game_system,
        on_exit_before_main_menu_system, on_exit_before_splash_screen_system,
        on_update_after_end_screen_system, on_update_after_in_game_system,
        on_update_after_main_menu_system, on_update_after_splash_screen_system,
        on_update_before_end_screen_system, on_update_before_in_game_system,
        on_update_before_main_menu_system, on_update_before_splash_screen_system,
    },
    GameState, ScheduleQueue,
};
use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemLabel)]
pub struct ScheduleSystem;

/// A plugin for the schedule of the game.
pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::AssetLoading)
            .insert_resource(ScheduleQueue::default())
            // Before Splash Screen
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_enter_before_splash_screen_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::BeforeSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_update_before_splash_screen_system)
                    .with_system(fade_in_system.after(on_update_before_splash_screen_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::BeforeSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_exit_before_splash_screen_system),
            )
            // After Splash Screen
            .add_system_set(
                SystemSet::on_enter(GameState::AfterSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_enter_after_splash_screen_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::AfterSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_update_after_splash_screen_system)
                    .with_system(fade_out_system.after(on_update_after_splash_screen_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterSplashScreen)
                    .label(ScheduleSystem)
                    .with_system(on_exit_after_splash_screen_system),
            )
            // Before Main Menu
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_enter_before_main_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::BeforeMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_update_before_main_menu_system)
                    .with_system(fade_in_system.after(on_update_before_main_menu_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::BeforeMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_exit_before_main_menu_system),
            )
            // After Main Menu
            .add_system_set(
                SystemSet::on_enter(GameState::AfterMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_enter_after_main_menu_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::AfterMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_update_after_main_menu_system)
                    .with_system(fade_out_system.after(on_update_after_main_menu_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterMainMenu)
                    .label(ScheduleSystem)
                    .with_system(on_exit_after_main_menu_system),
            )
            // Before In Game
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeInGame)
                    .label(ScheduleSystem)
                    .with_system(on_enter_before_in_game_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::BeforeInGame)
                    .label(ScheduleSystem)
                    .with_system(on_update_before_in_game_system)
                    .with_system(fade_in_system.after(on_update_before_in_game_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::BeforeInGame)
                    .label(ScheduleSystem)
                    .with_system(on_exit_before_in_game_system),
            )
            // In Game
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .label(ScheduleSystem)
                    .with_system(check_for_paused_system),
            )
            // Paused
            .add_system_set(
                SystemSet::on_update(GameState::Paused)
                    .label(ScheduleSystem)
                    .with_system(check_for_unpaused_system),
            )
            // After In Game
            .add_system_set(
                SystemSet::on_enter(GameState::AfterInGame)
                    .label(ScheduleSystem)
                    .with_system(on_enter_after_in_game_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::AfterInGame)
                    .label(ScheduleSystem)
                    .with_system(on_update_after_in_game_system)
                    .with_system(fade_out_system.after(on_update_after_in_game_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterInGame)
                    .label(ScheduleSystem)
                    .with_system(on_exit_after_in_game_system),
            )
            // Before End Screen
            .add_system_set(
                SystemSet::on_enter(GameState::BeforeEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_enter_before_end_screen_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::BeforeEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_update_before_end_screen_system)
                    .with_system(fade_in_system.after(on_update_before_end_screen_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::BeforeEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_exit_before_end_screen_system),
            )
            // After End Screen
            .add_system_set(
                SystemSet::on_enter(GameState::AfterEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_enter_after_end_screen_system),
            )
            .add_system_set(
                SystemSet::on_update(GameState::AfterEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_update_after_end_screen_system)
                    .with_system(fade_out_system.after(on_update_after_end_screen_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::AfterEndScreen)
                    .label(ScheduleSystem)
                    .with_system(on_exit_after_end_screen_system),
            );
    }
}
